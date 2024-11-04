use super::*;

pub struct MainMenu {}

impl MainMenu {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for MainMenu {
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        center(Button::new(text!("Play")).on_press(Message::ChangeScreen(ScreenChoices::Game)))
            .into()
    }
}
