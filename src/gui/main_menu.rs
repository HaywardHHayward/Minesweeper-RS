use iced::{Element, Task, widget as GuiWidget};

use super::{Message as AppMessage, ScreenState, ScreenTrait};

pub struct MainMenu;

impl From<MainMenu> for ScreenState {
    fn from(main_menu: MainMenu) -> Self {
        ScreenState::MainMenu(main_menu)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Start,
    Settings,
    Exit,
}

impl From<Message> for AppMessage {
    fn from(message: Message) -> AppMessage {
        AppMessage::MainMenu(message)
    }
}

impl ScreenTrait for MainMenu {
    type Message = Message;

    fn update(&mut self, message: Message) -> Task<AppMessage> {
        match message {
            Message::Start => todo!(),
            Message::Settings => todo!(),
            Message::Exit => iced::exit(),
        }
    }
    fn view(&self) -> Element<Message> {
        let title = GuiWidget::text("Minesweeper").size(50);
        let buttons = GuiWidget::column![
            GuiWidget::button("Start").on_press(Message::Start),
            GuiWidget::button("Settings").on_press(Message::Settings),
            GuiWidget::button("Exit").on_press(Message::Exit),
        ]
        .spacing(5)
        .align_x(iced::Center);
        let display = GuiWidget::center(GuiWidget::column![title, buttons].align_x(iced::Center));
        display.into()
    }
}
