mod contract_greeting;

use iced::widget::{text, container};
use iced::{Application, Length, Settings, Theme, Element};

#[derive(Debug, Clone)]
enum Message {
    FetchLocalGreeting,
    FetchContractGreeting,
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
        String::from("SLEET HELLO")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::FetchLocalGreeting => {
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
            Message::FetchContractGreeting => {
                self.loading = true;
                iced::Command::perform(
                    async {
                        contract_greeting::fetch_and_save_contract_greeting()
                            .map_err(|e| e.to_string())?;
                        
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
        let local_button = iced::widget::button("Get Local Greeting")
            .padding(10)
            .style(iced::theme::Button::Primary);

        let contract_button = iced::widget::button("Get Contract Greeting")
            .padding(10)
            .style(iced::theme::Button::Secondary);

        let (local_button, contract_button) = if self.loading {
            (local_button.on_press_maybe(None), contract_button.on_press_maybe(None))
        } else {
            (local_button.on_press(Message::FetchLocalGreeting), 
             contract_button.on_press(Message::FetchContractGreeting))
        };

        let greeting_display = iced::widget::container(
            iced::widget::text(&self.greeting)
                .size(16)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
        )
        .padding(10)
        .style(iced::theme::Container::Box);

        let content = iced::widget::column![
            iced::widget::text("SLEET HELLO")
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
                    iced::widget::row![local_button, contract_button].spacing(10).into()
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

    let mut settings = Settings::default();
    settings.default_text_size = 20.0;
    settings.default_font = iced::Font::DEFAULT;
    settings.antialiasing = true;

    HelloApp::run(settings)
}
