use std::future::Future;
use iced::widget::{button, column, text};
use iced::{Application, Command, Element, Settings, Theme};
use iced::executor;
use std::io::Error;

mod config;
mod near;
mod near_test;

use near::NearClient;

#[derive(Debug)]
struct TokioExecutor {
    runtime: tokio::runtime::Runtime,
}

impl executor::Executor for TokioExecutor {
    fn new() -> Result<Self, Error> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| e)?;
        let _guard = runtime.enter();
        Ok(TokioExecutor { runtime })
    }

    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let handle = self.runtime.handle().clone();
        handle.spawn(future);
    }
}

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.default_text_size = 20.0;
    HelloApp::run(settings)
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
    type Executor = TokioExecutor;
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
                let client = self.near_client.as_ref().cloned();
                Command::perform(Self::load_greeting(client), Message::GreetingLoaded)
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
                let client = self.near_client.as_ref().cloned();
                Command::perform(Self::load_greeting(client), Message::GreetingLoaded)
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
    async fn load_greeting(client: Option<NearClient>) -> Result<String, String> {
        tokio::task::spawn(async move {
            if let Some(client) = client {
                client.get_greeting().await.map_err(|e| e.to_string())
            } else {
                Err("NEAR client not initialized".to_string())
            }
        }).await.unwrap_or_else(|e| Err(e.to_string()))
    }
}

