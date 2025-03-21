use iced::widget::{button, column, text};
use iced::{Application, Command, Element, Settings, Theme};
use iced::executor;

mod config;
mod near;

use near::NearClient;

pub fn main() -> iced::Result {
    HelloApp::run(Settings::default())
}

#[derive(Debug, Default)]
struct HelloApp {
    greeting: String,
    near_client: Option<NearClient>,
    error: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    Initialize,
    GreetingLoaded(Result<String, String>),
    RefreshGreeting,
}

impl Application for HelloApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::perform(async {}, |_| Message::Initialize))
    }

    fn title(&self) -> String {
        String::from("NEAR Hello - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Initialize => {
                self.near_client = Some(NearClient::new());
                Command::perform(Self::load_greeting(self.near_client.as_ref()), Message::GreetingLoaded)
            }
            Message::GreetingLoaded(result) => {
                match result {
                    Ok(greeting) => {
                        self.greeting = greeting;
                        self.error = None;
                    }
                    Err(error) => {
                        self.error = Some(error);
                    }
                }
                Command::none()
            }
            Message::RefreshGreeting => {
                Command::perform(Self::load_greeting(self.near_client.as_ref()), Message::GreetingLoaded)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = if let Some(error) = &self.error {
            text(format!("Error: {}", error)).size(20)
        } else if self.greeting.is_empty() {
            text("Loading...").size(20)
        } else {
            text(&self.greeting).size(30)
        };

        column![
            content,
            button("Refresh").on_press(Message::RefreshGreeting)
        ]
        .spacing(20)
        .into()
    }
}

impl HelloApp {
    async fn load_greeting(client: Option<&NearClient>) -> Result<String, String> {
        if let Some(client) = client {
            client.get_greeting().await.map_err(|e| e.to_string())
        } else {
            Err("NEAR client not initialized".to_string())
        }
    }
}
}
