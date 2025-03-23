mod contract_greeting;
mod ui;

use iced::{Application, Settings, Theme, Element};
use ui::HelloView;

#[derive(Debug, Clone)]
pub enum Message {
    FetchLocalGreeting,
    FetchContractGreeting,
    GreetingReceived(Result<String, String>),
}

struct HelloApp {
    view: HelloView,
}

impl Application for HelloApp {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (Self {
            view: HelloView::new(String::new(), false),
        }, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("SLEET HELLO")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::FetchLocalGreeting => {
                self.view = HelloView::new(self.view.greeting().to_string(), true);
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
                self.view = HelloView::new(self.view.greeting().to_string(), true);
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
                self.view = HelloView::new(greeting, false);
                iced::Command::none()
            }
            Message::GreetingReceived(Err(e)) => {
                self.view = HelloView::new(e, false);
                iced::Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        self.view.view()
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
