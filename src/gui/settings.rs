use iced::{Element, Task, widget as GuiWidget};
use serde::{Deserialize, Serialize};

use super::{Message as AppMessage, Screen, ScreenState, ScreenTrait};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    // Placeholder for user settings, theme, leaderboard save location, etc.
}

#[derive(Debug)]
pub struct Settings {
    user_settings: UserSettings,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            user_settings: UserSettings {}, // Placeholder for actual settings initialization
        }
    }
}

impl From<Settings> for ScreenState {
    fn from(settings: Settings) -> ScreenState {
        ScreenState::Settings(settings)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Exit,
}

impl From<Message> for AppMessage {
    fn from(value: Message) -> Self {
        AppMessage::Settings(value)
    }
}

impl ScreenTrait for Settings {
    type Message = Message;

    fn update(&mut self, message: Message) -> Task<AppMessage> {
        match message {
            Message::Exit => Task::done(AppMessage::ChangeScreen(Screen::MainMenu)),
        }
    }
    fn view(&self) -> Element<Self::Message> {
        let buttons = GuiWidget::column![GuiWidget::Button::new("Exit").on_press(Message::Exit),];
        let display = GuiWidget::center(
            GuiWidget::column![buttons].align_x(iced::alignment::Horizontal::Left),
        );
        display.into()
    }
}
