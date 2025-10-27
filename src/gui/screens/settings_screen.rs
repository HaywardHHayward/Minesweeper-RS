use std::sync::Arc;

use iced::{Element, Task, widget as GuiWidget};

use super::{AppMessage, Leaderboard, MainMenu, Message as SuperMessage};
use crate::{ArcLock, Config, GameTheme, MenuTheme, Screen};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    MenuThemeChanged(MenuTheme),
    GameThemeChanged(GameTheme),
    ScaleFactorChanged(f32),
    ApplyChanges,
    ResetChanges,
    LeaderboardReset(LeaderboardReset),
}

#[derive(Debug, Clone)]
pub enum LeaderboardReset {
    Prompt,
    Confirm,
    Cancel,
}

#[derive(Debug)]
pub struct SettingsScreen {
    config: ArcLock<Config>,
    menu_theme: Option<MenuTheme>,
    game_theme: Option<GameTheme>,
    scale_factor: Option<f32>,
    showing_confirmation: bool,
}

impl SettingsScreen {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self {
            config,
            menu_theme: None,
            game_theme: None,
            scale_factor: None,
            showing_confirmation: false,
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
            Message::Back => Some(
                Task::perform(async { MainMenu::build(config) }, move |item| {
                    Arc::new(Box::new(item) as Box<dyn Screen>)
                })
                .map(AppMessage::ChangeScreen)
                .map(SuperMessage::App),
            ),
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
            Message::LeaderboardReset(action) => match action {
                LeaderboardReset::Prompt => {
                    self.showing_confirmation = true;
                    None
                }
                LeaderboardReset::Confirm => {
                    self.showing_confirmation = false;
                    if let Err(error) = Leaderboard::delete_entries() {
                        eprintln!("Failed to reset leaderboard: {error}");
                    }
                    None
                }
                LeaderboardReset::Cancel => {
                    self.showing_confirmation = false;
                    None
                }
            },
        }
    }
    fn view(&self) -> Element<'_, SuperMessage> {
        let menu_theme = &self.config.read().unwrap().menu_theme;

        let popup_window = if self.showing_confirmation {
            let confirmation_text = menu_theme.text(
                "Are you sure you want to reset the leaderboard? This action cannot be undone.",
            );
            let confirm_button = menu_theme
                .button(menu_theme.text("Confirm"), crate::MenuButtonStyle::Danger)
                .on_press(SuperMessage::SettingsScreen(Message::LeaderboardReset(
                    LeaderboardReset::Confirm,
                )));
            let cancel_button = menu_theme
                .button(menu_theme.text("Cancel"), crate::MenuButtonStyle::Secondary)
                .on_press(SuperMessage::SettingsScreen(Message::LeaderboardReset(
                    LeaderboardReset::Cancel,
                )));
            let buttons = GuiWidget::row![confirm_button, cancel_button].spacing(10);
            let content = GuiWidget::column![confirmation_text, buttons]
                .spacing(20)
                .align_x(iced::Center);
            Some(GuiWidget::opaque(
                GuiWidget::container(content)
                    .padding(20)
                    .style(GuiWidget::container::bordered_box),
            ))
        } else {
            None
        };

        let menu_theme_text = menu_theme.text("Menu Theme:");
        let menu_theme_picker =
            GuiWidget::pick_list(MenuTheme::ALL, self.menu_theme.to_owned(), |theme| {
                SuperMessage::SettingsScreen(Message::MenuThemeChanged(theme))
            })
            .font(menu_theme.default_font())
            .placeholder(self.config.read().unwrap().menu_theme.to_string());
        let menu_theme_row = GuiWidget::row![menu_theme_text, menu_theme_picker].spacing(10);

        let game_theme_text = menu_theme.text("Game Theme:");
        let game_theme_picker =
            GuiWidget::pick_list(GameTheme::ALL, self.game_theme.to_owned(), |theme| {
                SuperMessage::SettingsScreen(Message::GameThemeChanged(theme))
            })
            .font(menu_theme.default_font())
            .placeholder(self.config.read().unwrap().game_theme.to_string());
        let game_theme = GuiWidget::row![game_theme_text, game_theme_picker].spacing(10);

        let scale_factor_text = menu_theme.text("Scale Factor:");
        let scale_factor_slider = GuiWidget::slider(
            0.25..=3.0,
            self.scale_factor
                .unwrap_or_else(|| self.config.read().unwrap().scale_factor),
            |value| SuperMessage::SettingsScreen(Message::ScaleFactorChanged(value)),
        )
        .step(0.25);
        let scale_factor_value = menu_theme.text(format!(
            "{:.2}x",
            self.scale_factor
                .unwrap_or_else(|| self.config.read().unwrap().scale_factor)
        ));
        let scale_factor =
            GuiWidget::row![scale_factor_text, scale_factor_slider, scale_factor_value].spacing(10);

        let reset_leaderboard_button = menu_theme
            .button(
                menu_theme.text("Reset Leaderboard"),
                crate::MenuButtonStyle::Danger,
            )
            .on_press(SuperMessage::SettingsScreen(Message::LeaderboardReset(
                LeaderboardReset::Prompt,
            )));

        let settings_column = GuiWidget::column![
            menu_theme_row,
            game_theme,
            scale_factor,
            reset_leaderboard_button
        ]
        .spacing(10);

        let apply_button = menu_theme
            .button(
                menu_theme.text("Apply Changes"),
                crate::MenuButtonStyle::Primary,
            )
            .on_press(SuperMessage::SettingsScreen(Message::ApplyChanges));
        let reset_button = menu_theme
            .button(
                menu_theme.text("Reset Changes"),
                crate::MenuButtonStyle::Danger,
            )
            .on_press(SuperMessage::SettingsScreen(Message::ResetChanges));
        let back_button = menu_theme
            .button(menu_theme.text("Back"), crate::MenuButtonStyle::Secondary)
            .on_press(SuperMessage::SettingsScreen(Message::Back));

        let buttons = GuiWidget::row![apply_button, reset_button, back_button].spacing(10);

        let settings_content = GuiWidget::column![settings_column, buttons]
            .width(iced::Shrink)
            .align_x(iced::Center)
            .spacing(20);

        if let Some(popup) = popup_window {
            GuiWidget::stack![
                GuiWidget::center(settings_content),
                GuiWidget::center(popup),
            ]
            .into()
        } else {
            GuiWidget::center(settings_content).into()
        }
    }
}
