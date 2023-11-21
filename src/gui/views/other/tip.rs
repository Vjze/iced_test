use iced::{
    alignment::{self, Horizontal},
    widget::{button, container, row, text, Container},
    Length,
};
use iced_aw::{modal, Card};

use crate::gui::{Message, MyTools};

pub fn tip(
    state: &MyTools,
    c: Container<'static, Message>,
    body: &str,
) -> Container<'static, Message> {
    let overlay = if state.show_modal {
        Some(
            Card::new(
                text("提示").horizontal_alignment(Horizontal::Center),
                text(format!("{}", body)).horizontal_alignment(Horizontal::Center),
            )
            .foot(
                row![
                    button(text("确认").horizontal_alignment(Horizontal::Center),)
                        .width(Length::Fill)
                        .on_press(Message::CloseModal),
                ]
                .spacing(10)
                .padding(5)
                .width(Length::Fill),
            )
            .max_width(300.0)
            .on_close(Message::CloseModal),
        )
    } else {
        None
    };
    let tip = modal(c, overlay)
        .backdrop(Message::CloseModal)
        .on_esc(Message::CloseModal)
        .align_y(alignment::Vertical::Center);

    container(tip).into()
}
