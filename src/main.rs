use iced::widget::{text, container};
use iced::{Application, Length, Settings, Theme, Element};
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
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
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
                iced::Command::perform(
                    async {
                        match std::fs::read_to_string("config/greeting.json") {
                            Ok(content) => {
                                let greeting_data: serde_json::Value = serde_json::from_str(&content)
                                    .map_err(|e| format!("Failed to parse greeting JSON: {}", e))?;
                                Ok(greeting_data["greeting"].as_str()
                                    .ok_or("Missing greeting field")?                                    
                                    .to_string())
                            }
                            Err(e) => Err(format!("Failed to read greeting file: {}", e))
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
                    Into::<Element<Message>>::into(
                        container(
                            text("Loading...")
                                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.4, 0.4, 0.4)))
                        ).width(Length::Fill)
                    )
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
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = rt.enter();
    
    // Keep the runtime alive without spawning unnecessary tasks
    std::thread::spawn(move || {
        rt.block_on(async {
            tokio::signal::ctrl_c().await.ok();
        });
    });

    let mut settings = Settings::default();
    settings.default_text_size = 20.0;
    settings.default_font = iced::Font::DEFAULT;
    settings.antialiasing = true;

    HelloApp::run(settings)
}
