use std::{ops::Deref, sync::Arc};

use iced::{Element, Task, widget as GuiWidget};

use super::{AppMessage, Message as SuperMessage};
use crate::{ArcLock, Config, Screen};

#[derive(Debug, Clone)]
pub enum Message {
    ToGameSelection,
    ToSettings,
    ToAbout,
    Quit,
}

#[derive(Debug)]
pub struct MainMenu {
    config: ArcLock<Config>,
}

impl MainMenu {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self { config }
    }
}

impl Screen for MainMenu {
    fn update(&mut self, message: SuperMessage) -> Option<Task<SuperMessage>> {
        let SuperMessage::MainMenu(message) = message else {
            return None;
        };
        let config = self.config.clone();
        match message {
            Message::ToGameSelection => {
                Some(Task::done(SuperMessage::App(AppMessage::ChangeScreen(
                    Arc::new(move || Box::new(super::GameSelection::build(config.clone()))),
                ))))
            }
            Message::ToSettings => Some(Task::done(SuperMessage::App(AppMessage::ChangeScreen(
                Arc::new(move || Box::new(super::SettingsScreen::build(config.clone()))),
            )))),
            Message::ToAbout => Some(Task::done(SuperMessage::App(AppMessage::ChangeScreen(
                Arc::new(move || Box::new(super::About::build(config.clone()))),
            )))),
            Message::Quit => Some(Task::done(SuperMessage::App(AppMessage::CloseApp))),
        }
    }
    fn view(&self) -> Element<'_, SuperMessage> {
        let title_text = GuiWidget::text("Minesweeper").size(50);
        let author_text = GuiWidget::text("by Hayward H. Hayward").size(20);
        let main_title = GuiWidget::column![title_text, author_text].align_x(iced::Center);

        let play_button = GuiWidget::button("Play")
            .on_press(SuperMessage::MainMenu(Message::ToGameSelection))
            .style(GuiWidget::button::primary);
        let settings_button = GuiWidget::button("Settings")
            .on_press(SuperMessage::MainMenu(Message::ToSettings))
            .style(GuiWidget::button::secondary);
        let about_button = GuiWidget::button("About")
            .on_press(SuperMessage::MainMenu(Message::ToAbout))
            .style(GuiWidget::button::secondary);
        let quit_button = GuiWidget::button("Quit")
            .on_press(SuperMessage::MainMenu(Message::Quit))
            .style(GuiWidget::button::danger);
        let buttons = GuiWidget::column![play_button, settings_button, about_button, quit_button]
            .width(iced::Shrink)
            .spacing(5)
            .align_x(iced::Center);

        let main_menu = GuiWidget::column![main_title, buttons]
            .spacing(30)
            .align_x(iced::Center);
        GuiWidget::center(main_menu).into()
    }
}
