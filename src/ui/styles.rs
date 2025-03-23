use iced::{Color, Theme};

pub fn title_text_color() -> Color {
    Color::from_rgb(0.1, 0.1, 0.4)
}

pub fn loading_text_color() -> Color {
    Color::from_rgb(0.4, 0.4, 0.4)
}

pub fn greeting_container_style() -> iced::theme::Container {
    iced::theme::Container::Box
}