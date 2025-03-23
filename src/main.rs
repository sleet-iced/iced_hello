use iced::widget::container;
use iced::widget::image::Image;
use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use std::path::PathBuf;

use near_api::JsonRpcConnection;
use serde_json::json;
use std::sync::Arc;

#[derive(Debug, Clone)]
enum Message {
    FetchGreeting,
    GreetingReceived(Result<String, String>),
}

struct HelloApp {
    greeting: String,
    loading: bool,
}

impl iced::Application for HelloApp {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();
    
    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (Self {
            greeting: String::new(),
            loading: false,
        }, iced::Command::none())
    }
    
    fn title(&self) -> String {
        String::from("Hello Greeter")
    }
    
    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::FetchGreeting => {
                self.loading = true;
                let config = std::fs::read_to_string("config/config.json")
                    .expect("Missing config.json");
                let config: serde_json::Value = serde_json::from_str(&config).unwrap();

                iced::Command::perform(
                    async move {
                        let rpc = JsonRpcConnection::new(config["rpc_url"].as_str().unwrap());
                        let result = rpc
                            .call(
                                "query",
                                json!({
                                    "request_type": "call_function",
                                    "account_id": config["contract_id"],
                                    "method_name": "get_greeting",
                                    "args_base64": "",
                                    "finality": "optimistic"
                                })
                            )
                            .await;

                        result
                            .map(|res| String::from_utf8_lossy(&res.result).into_owned())
                            .map_err(|e| format!("Error: {}", e))
                    },
                    Message::GreetingReceived,
                )
            }
            Message::GreetingReceived(Ok(greeting)) => {
                self.greeting = greeting;
                self.loading = false;
                iced::Command::none()
            }
            Message::GreetingReceived(Err(e)) => {
                self.greeting = e;
                self.loading = false;
                iced::Command::none()
            }
        }
    }
    
    fn view(&self) -> iced::Element<'_, Message> {
        let button = iced::widget::button("Get Greeting")
            .padding(10)
            .style(iced::theme::Button::Primary);

        let button = if self.loading {
            button.on_press_maybe(None)
        } else {
            button.on_press(Message::FetchGreeting)
        };

        let greeting_display = iced::widget::container(
            iced::widget::text(&self.greeting)
                .size(16)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
        )
        .padding(10)
        .style(iced::theme::Container::Box);

        let content = iced::widget::column![
            iced::widget::text("Hello Greeter")
                .size(28)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.1, 0.1, 0.4))),
            iced::widget::row![
                if self.loading {
                    iced::widget::container(
                        iced::widget::text("Loading...")
                            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.4, 0.4, 0.4)))
                    ).into()
                } else {
                    button
                },
            ].spacing(10),
            greeting_display.width(Length::Units(300))
        ]
        .spacing(20)
        .padding(20)
        .align_items(iced::Alignment::Center);

        iced::widget::container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.default_text_size = 20.0;
    HelloApp::run(settings)
}


}