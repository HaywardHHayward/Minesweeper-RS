use iced::{Element, Task, widget as GuiWidget};

use super::{Message as AppMessage, Screen, ScreenState, ScreenTrait};

#[derive(Debug, Default)]
pub struct MainMenu;

impl From<MainMenu> for ScreenState {
    fn from(main_menu: MainMenu) -> Self {
        ScreenState::MainMenu(main_menu)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Start,
    OpenLeaderboard,
    OpenSettings,
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
            Message::OpenLeaderboard => todo!(),
            Message::OpenSettings => Task::done(AppMessage::ChangeScreen(Screen::Settings)),
            Message::Exit => iced::exit(),
        }
    }
    fn view(&self) -> Element<Message> {
        let title = GuiWidget::text("Minesweeper").size(50);
        let buttons = GuiWidget::column![
            GuiWidget::button("Start").on_press(Message::Start),
            GuiWidget::button("Leaderboard").on_press(Message::OpenLeaderboard),
            GuiWidget::button("Settings").on_press(Message::OpenSettings),
            GuiWidget::button("Exit").on_press(Message::Exit),
        ]
        .spacing(5)
        .align_x(iced::Center);
        let display = GuiWidget::center(GuiWidget::column![title, buttons].align_x(iced::Center));
        display.into()
    }
}
