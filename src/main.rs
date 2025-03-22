use iced::widget::{container, text};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.default_text_size = 20.0;
    HelloApp::run(settings)
}

#[derive(Debug, Default)]
struct HelloApp;

#[derive(Debug, Clone)]
enum Message {}

impl Application for HelloApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Sleet")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content = text("❄️").size(50);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(CustomStyle)))
            .into()
    }
}

struct CustomStyle;

impl container::StyleSheet for CustomStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0x7d as f32 / 255.0,
                0x84 as f32 / 255.0,
                0xb2 as f32 / 255.0,
            ))),
            ..Default::default()
        }
    }
}

