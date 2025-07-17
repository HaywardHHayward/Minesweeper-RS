use iced::{Element, Task, widget as GuiWidget};

use crate::gui::{Application, Message as AppMessage, ScreenTrait, ScreenType, config::*};
#[derive(Debug)]
pub(crate) struct SettingsScreen {
    config: Config,
}

#[derive(Debug, Clone)]
pub(crate) enum Action {
    ReturnToMainMenu,
    MenuThemeSelected(MenuTheme),
    GameThemeSelected(GameTheme),
}

impl SettingsScreen {
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }
}

impl ScreenTrait for SettingsScreen {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<AppMessage> {
        match message {
            Self::Message::ReturnToMainMenu => {
                self.config
                    .save(&Application::app_dirs().config_dir().join("config.yaml"));
                Task::done(AppMessage::ChangeScreen(ScreenType::MainMenu))
            }
            Self::Message::MenuThemeSelected(theme) => {
                self.config.update_menu_theme(theme);
                Task::none()
            }
            Self::Message::GameThemeSelected(game) => {
                self.config.update_game_theme(game);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let menu_themes = [MenuTheme::Light];
        let menu_theme = GuiWidget::pick_list(
            menu_themes,
            Some(self.config.get_menu_theme()),
            Self::Message::MenuThemeSelected,
        );
        let game_themes = [GameTheme::Default];
        let game_theme = GuiWidget::pick_list(
            game_themes,
            Some(self.config.get_game_theme()),
            Self::Message::GameThemeSelected,
        );
        let options = GuiWidget::column![menu_theme, game_theme,].align_x(iced::Center);

        let buttons =
            GuiWidget::button("Return to Main Menu").on_press(Self::Message::ReturnToMainMenu);
        let content = GuiWidget::column![options, buttons]
            .spacing(20)
            .align_x(iced::Center);
        GuiWidget::center(content).into()
    }
}
