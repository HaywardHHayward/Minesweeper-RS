use std::sync::Arc;

use iced::{Element, Task, widget as GuiWidget};

use super::{AppMessage, MainMenu, Message as SuperMessage};
use crate::{ArcLock, Config, Screen};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
}

#[derive(Debug)]
pub struct SettingsScreen {
    config: ArcLock<Config>,
}

impl SettingsScreen {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self { config }
    }
}

impl Screen for SettingsScreen {
    fn update(&mut self, message: SuperMessage) -> Option<Task<SuperMessage>> {
        let SuperMessage::SettingsScreen(message) = message else {
            return None;
        };
        let config = self.config.clone();
        match message {
            Message::Back => Some(Task::perform(
                async { MainMenu::build(config) },
                move |item| SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item)))),
            )),
        }
    }
    fn view(&self) -> Element<'_, SuperMessage> {
        let todo_message = GuiWidget::text("Settings screen is under construction!");
        let back_button = GuiWidget::button("Back")
            .on_press(SuperMessage::SettingsScreen(Message::Back))
            .style(GuiWidget::button::secondary);
        let content = GuiWidget::column![todo_message, back_button]
            .align_x(iced::Center)
            .spacing(20);
        GuiWidget::center(content).into()
    }
}
