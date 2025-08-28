use std::sync::Arc;

use iced::{Task, widget as GuiWidget};

use super::{AppMessage, CustomSetup, Game, MainMenu, Message as SuperMessage};
use crate::{ArcLock, Board, Config, Screen};

#[derive(Debug, Clone)]
pub enum Message {
    BeginnerSelected,
    IntermediateSelected,
    ExpertSelected,
    CustomSelected,
    Back,
}

#[derive(Debug)]
pub struct GameSelection {
    config: ArcLock<Config>,
}

impl GameSelection {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self { config }
    }
}

impl Screen for GameSelection {
    fn update(&mut self, message: SuperMessage) -> Option<Task<SuperMessage>> {
        let SuperMessage::GameSelection(message) = message else {
            return None;
        };
        let config = self.config.clone();
        match message {
            Message::BeginnerSelected => Some(Task::perform(
                async { Game::build(config, Board::create_beginner()) },
                move |item| SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item)))),
            )),
            Message::IntermediateSelected => Some(Task::perform(
                async { Game::build(config, Board::create_intermediate()) },
                move |item| SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item)))),
            )),
            Message::ExpertSelected => Some(Task::perform(
                async { Game::build(config, Board::create_expert()) },
                move |item| SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item)))),
            )),
            Message::CustomSelected => Some(Task::perform(
                async { CustomSetup::build(config) },
                move |item| SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item)))),
            )),
            Message::Back => Some(Task::perform(
                async { MainMenu::build(config) },
                move |item| SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item)))),
            )),
        }
    }
    fn view(&self) -> iced::Element<'_, SuperMessage> {
        let beginner_button = GuiWidget::button("Beginner (9x9, 10 mines)")
            .on_press(SuperMessage::GameSelection(Message::BeginnerSelected))
            .style(|theme, status| {
                self.config
                    .read()
                    .unwrap()
                    .menu_theme
                    .button_style(crate::gui::config::MenuButtonStyle::Primary)(
                    theme, status
                )
            });
        let intermediate_button = GuiWidget::button("Intermediate (16x16, 40 mines)")
            .on_press(SuperMessage::GameSelection(Message::IntermediateSelected))
            .style(|theme, status| {
                self.config
                    .read()
                    .unwrap()
                    .menu_theme
                    .button_style(crate::gui::config::MenuButtonStyle::Primary)(
                    theme, status
                )
            });
        let expert_button = GuiWidget::button("Expert (30x16, 99 mines)")
            .on_press(SuperMessage::GameSelection(Message::ExpertSelected))
            .style(|theme, status| {
                self.config
                    .read()
                    .unwrap()
                    .menu_theme
                    .button_style(crate::gui::config::MenuButtonStyle::Primary)(
                    theme, status
                )
            });
        let custom_button = GuiWidget::button("Custom")
            .on_press(SuperMessage::GameSelection(Message::CustomSelected))
            .style(|theme, status| {
                self.config
                    .read()
                    .unwrap()
                    .menu_theme
                    .button_style(crate::gui::config::MenuButtonStyle::Primary)(
                    theme, status
                )
            });

        let buttons = GuiWidget::column![
            beginner_button,
            intermediate_button,
            expert_button,
            custom_button
        ]
        .spacing(10)
        .align_x(iced::Center);

        let back_button = GuiWidget::button("Back")
            .on_press(SuperMessage::GameSelection(Message::Back))
            .style(|theme, status| {
                self.config
                    .read()
                    .unwrap()
                    .menu_theme
                    .button_style(crate::gui::config::MenuButtonStyle::Secondary)(
                    theme, status
                )
            });
        let content = GuiWidget::column![buttons, back_button]
            .align_x(iced::Center)
            .spacing(20);
        GuiWidget::center(content).into()
    }
}
