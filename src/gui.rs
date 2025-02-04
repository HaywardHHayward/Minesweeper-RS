pub mod main_menu;

#[derive(Debug)]
pub struct Application {}

#[derive(Debug, Copy, Clone)]
pub enum Message {}

impl Application {
    pub fn initialize() -> Self {
        Self {}
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::initialize()
    }
}

pub fn update(_: &mut Application, _: Message) -> impl Into<iced::Task<Message>> {
    iced::Task::none()
}

pub fn view(_: &Application) -> iced::Element<Message> {
    iced::widget::text("Hello world!").into()
}
