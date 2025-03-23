use iced::{widget::text, widget::button, widget::container, widget::column, widget::row};
use iced::{Application, Length, Settings, Theme, Alignment, alignment, Element, Renderer, executor};
use serde_json::json;
use reqwest;

#[derive(Debug, Clone)]
enum Message {
    FetchGreeting,
    GreetingReceived(Result<String, String>),
}

struct HelloApp {
    greeting: String,
    loading: bool,
}

impl Application for HelloApp {
    type Theme = Theme;
    type Renderer = Renderer;
    type Executor = executor::Default;
    type Message = Message;
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
                        let response = reqwest::Client::new()
                            .post(config["rpc_url"].as_str().unwrap())
                            .json(&json!({
                                "jsonrpc": "2.0",
                                "id": "dontcare",
                                "method": "query",
                                "params": {
                                    "request_type": "call_function",
                                    "account_id": config["contract"],
                                    "method_name": "get_greeting",
                                    "args_base64": "",
                                    "finality": "final"
                                }
                            }))
                            .send()
                            .await;

                        match response {
                            Ok(res) => {
                                let response_text = res.text().await.unwrap_or_default();
                                let response_data: serde_json::Value = serde_json::from_str(&response_text)
                                    .map_err(|e| format!("Failed to parse JSON response: {}", e))?;
                                let result = response_data["result"]["result"]
                                    .as_array()
                                    .ok_or("Invalid response format")?;
                                let greeting_bytes: Vec<u8> = result
                                    .iter()
                                    .map(|v| v.as_u64().unwrap_or_default() as u8)
                                    .collect();
                                Ok(String::from_utf8_lossy(&greeting_bytes).into_owned())
                            }
                            Err(e) => Err(format!("Request failed: {}", e))
                        }
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
        let button: iced::widget::Button<'_, Message, iced::Renderer> = iced::widget::button("Get Greeting")
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
                    container(
                        text("Loading...")
                            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.4, 0.4, 0.4)))
                    ).width(Length::Fill).into()
                } else {
                    button.into()
                }
            ].spacing(10),
            greeting_display.width(Length::Fixed(300.0))
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
