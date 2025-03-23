use iced::widget::{text, container, button, column, row};
use iced::{Length, Element, Alignment};
use crate::Message;
use super::styles::*;

pub struct HelloView {
    greeting: String,
    loading: bool,
    contract_loading: bool,
}

impl HelloView {
    pub fn greeting(&self) -> &str {
        &self.greeting
    }
}

impl HelloView {
    pub fn new(greeting: String, loading: bool) -> Self {
        Self { greeting, loading, contract_loading: false }
    }

    pub fn view(&self) -> Element<Message> {
        let local_button = button("Get Local Greeting")
            .padding(10)
            .style(iced::theme::Button::Primary);

        let contract_button = button("Get Contract Greeting")
            .padding(10)
            .style(iced::theme::Button::Secondary);

        let local_button = if self.loading {
            local_button.on_press_maybe(None)
        } else {
            local_button.on_press(Message::FetchLocalGreeting)
        };

        let contract_button = if self.contract_loading {
            contract_button.on_press_maybe(None)
        } else {
            contract_button.on_press(Message::FetchContractGreeting)
        };

        let greeting_display = container(
            text(&self.greeting)
                .size(16)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
        )
        .padding(10)
        .style(super::styles::greeting_container_style());

        let content = column![
            text("SLEET HELLO")
                .size(28)
                .style(iced::theme::Text::Color(title_text_color())),
            row![
                row![local_button, contract_button].spacing(10),
            if self.loading {
                Into::<Element<Message>>::into(
                    text("Loading local...")
                        .style(iced::theme::Text::Color(loading_text_color()))
                )
            } else if self.contract_loading {
                Into::<Element<Message>>::into(
                    text("Loading contract...")
                        .style(iced::theme::Text::Color(loading_text_color()))
                )
            } else {
                Into::<Element<Message>>::into(text(""))
            }
            ].spacing(10),
            greeting_display.width(Length::Fixed(300.0))
        ]
        .spacing(20)
        .padding(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}