use std::sync::Arc;

use iced::{Element, Task, widget as GuiWidget};

use super::{AppMessage, MainMenu, Message as SuperMessage};
use crate::{
    ArcLock, Config, Screen,
    gui::config::{GameTheme, MenuTheme},
};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    MenuThemeChanged(MenuTheme),
    GameThemeChanged(GameTheme),
    ScaleFactorChanged(f64),
    ApplyChanges,
    ResetChanges,
}

#[derive(Debug)]
pub struct SettingsScreen {
    config: ArcLock<Config>,
    menu_theme: Option<MenuTheme>,
    game_theme: Option<GameTheme>,
    scale_factor: Option<f64>,
}

impl SettingsScreen {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self {
            config,
            menu_theme: None,
            game_theme: None,
            scale_factor: None,
        }
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
            Message::MenuThemeChanged(theme) => {
                self.menu_theme = Some(theme);
                None
            }
            Message::GameThemeChanged(theme) => {
                self.game_theme = Some(theme);
                None
            }
            Message::ScaleFactorChanged(factor) => {
                self.scale_factor = Some(factor);
                None
            }
            Message::ApplyChanges => {
                let mut config_write = self.config.write().unwrap();
                if let Some(ref menu_theme) = self.menu_theme {
                    config_write.menu_theme = menu_theme.to_owned();
                }
                if let Some(ref game_theme) = self.game_theme {
                    config_write.game_theme = game_theme.to_owned();
                }
                if let Some(scale_factor) = self.scale_factor {
                    config_write.scale_factor = scale_factor;
                }
                Some(Task::done(SuperMessage::SettingsScreen(
                    Message::ResetChanges,
                )))
            }
            Message::ResetChanges => {
                self.menu_theme = None;
                self.game_theme = None;
                self.scale_factor = None;
                None
            }
        }
    }
    fn view(&self) -> Element<'_, SuperMessage> {
        let menu_theme_text = GuiWidget::text("Menu Theme:");
        let menu_theme_picker =
            GuiWidget::pick_list(MenuTheme::ALL, self.menu_theme.to_owned(), |theme| {
                SuperMessage::SettingsScreen(Message::MenuThemeChanged(theme))
            })
            .placeholder(self.config.read().unwrap().menu_theme.to_string());
        let menu_theme = GuiWidget::row![menu_theme_text, menu_theme_picker].spacing(10);

        let game_theme_text = GuiWidget::text("Game Theme:");
        let game_theme_picker =
            GuiWidget::pick_list(GameTheme::ALL, self.game_theme.to_owned(), |theme| {
                SuperMessage::SettingsScreen(Message::GameThemeChanged(theme))
            })
            .placeholder(self.config.read().unwrap().game_theme.to_string());
        let game_theme = GuiWidget::row![game_theme_text, game_theme_picker].spacing(10);

        let scale_factor_text = GuiWidget::text("Scale Factor:");
        let scale_factor_slider = GuiWidget::slider(
            0.25..=3.0,
            self.scale_factor
                .unwrap_or_else(|| self.config.read().unwrap().scale_factor),
            |value| SuperMessage::SettingsScreen(Message::ScaleFactorChanged(value)),
        )
        .step(0.25);
        let scale_factor_value = GuiWidget::text(format!(
            "{:.2}x",
            self.scale_factor
                .unwrap_or_else(|| self.config.read().unwrap().scale_factor)
        ));
        let scale_factor =
            GuiWidget::row![scale_factor_text, scale_factor_slider, scale_factor_value].spacing(10);

        let settings_column = GuiWidget::column![menu_theme, game_theme, scale_factor].spacing(10);

        let menu_theme = &self.config.read().unwrap().menu_theme;
        let apply_button = menu_theme
            .button("Apply Changes", crate::MenuButtonStyle::Primary)
            .on_press(SuperMessage::SettingsScreen(Message::ApplyChanges));
        let reset_button = menu_theme
            .button("Reset Changes", crate::MenuButtonStyle::Danger)
            .on_press(SuperMessage::SettingsScreen(Message::ResetChanges));
        let back_button = menu_theme
            .button("Back", crate::MenuButtonStyle::Secondary)
            .on_press(SuperMessage::SettingsScreen(Message::Back));

        let buttons = GuiWidget::row![apply_button, reset_button, back_button].spacing(10);

        let content = GuiWidget::column![settings_column, buttons]
            .width(iced::Shrink)
            .align_x(iced::Center)
            .spacing(20);

        GuiWidget::center(content).into()
    }
}
