use std::sync::Arc;

use iced::{Element, Task, widget as GuiWidget};

use super::{About, AppMessage, GameSelection, Message as SuperMessage, SettingsScreen};
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
            Message::ToGameSelection => Some(Task::perform(
                async { GameSelection::build(config) },
                move |item| SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item)))),
            )),
            Message::ToSettings => Some(Task::perform(
                async { SettingsScreen::build(config) },
                move |item| SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item)))),
            )),
            Message::ToAbout => Some(Task::perform(async { About::build(config) }, move |item| {
                SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item))))
            })),
            Message::Quit => Some(Task::done(SuperMessage::App(AppMessage::CloseApp))),
        }
    }
    fn view(&self) -> Element<'_, SuperMessage> {
        let title_text = GuiWidget::text("Minesweeper").size(50);
        let author_text = GuiWidget::text("by Hayward H. Hayward").size(20);
        let main_title = GuiWidget::column![title_text, author_text].align_x(iced::Center);

        let menu_theme = &self.config.read().unwrap().menu_theme;

        let play_button = menu_theme
            .button("Play", crate::gui::config::MenuButtonStyle::Primary)
            .on_press(SuperMessage::MainMenu(Message::ToGameSelection));
        let settings_button = menu_theme
            .button("Settings", crate::gui::config::MenuButtonStyle::Secondary)
            .on_press(SuperMessage::MainMenu(Message::ToSettings));
        let about_button = menu_theme
            .button("About", crate::gui::config::MenuButtonStyle::Secondary)
            .on_press(SuperMessage::MainMenu(Message::ToAbout));
        let quit_button = menu_theme
            .button("Quit", crate::gui::config::MenuButtonStyle::Danger)
            .on_press(SuperMessage::MainMenu(Message::Quit));

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
