use iced::{Element, Task};

#[derive(Debug)]
pub enum Message {}

pub fn update(state: &mut (), message: Message) -> Task<Message> {
    Task::none()
}

pub fn view(state: &()) -> Element<Message> {
    iced::widget::text("Hello world!").into()
}
