use std::{fmt::Display, fs::File, path::Path};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub(crate) struct Config {
    theme: Theme,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
struct Theme {
    game_theme: GameTheme,
    menu_theme: MenuTheme,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub(crate) enum GameTheme {
    Default,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub(crate) enum MenuTheme {
    Light,
    // TODO: Dark
}

impl Display for GameTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GameTheme::Default => "Default",
        })
    }
}

impl Display for MenuTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MenuTheme::Light => "Light",
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: Theme {
                game_theme: GameTheme::Default,
                menu_theme: MenuTheme::Light,
            },
        }
    }
}

impl Config {
    pub(crate) fn save(&self, save_location: &Path) {
        let save_file = File::create(save_location).expect("Failed to create config file");
        serde_yml::to_writer(save_file, &self).expect("Failed to serialize config");
    }

    pub(crate) fn load(load_location: &Path) -> Result<Self, serde_yml::Error> {
        let config_file = File::open(load_location).expect("Failed to open config file");
        let config = serde_yml::from_reader(config_file)?;
        Ok(config)
    }

    pub(crate) fn update_menu_theme(&mut self, menu_theme: MenuTheme) {
        self.theme.menu_theme = menu_theme;
    }

    pub(crate) fn get_menu_theme(&self) -> &MenuTheme {
        &self.theme.menu_theme
    }

    pub(crate) fn update_game_theme(&mut self, game_theme: GameTheme) {
        self.theme.game_theme = game_theme;
    }

    pub(crate) fn get_game_theme(&self) -> &GameTheme {
        &self.theme.game_theme
    }
}
