use std::{fmt::Display, fs::File, path::Path};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Copy)]
pub(crate) struct Config {
    theme: Theme,
    scale_factor: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Copy)]
struct Theme {
    game_theme: GameTheme,
    menu_theme: MenuTheme,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Copy, PartialEq)]
pub(crate) enum GameTheme {
    SimpleLight,
    SimpleDark,
    Classic,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Copy, PartialEq)]
pub(crate) enum MenuTheme {
    Light,
    Dark,
}

impl Display for GameTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GameTheme::SimpleLight => "Simple (Light)",
            GameTheme::SimpleDark => "Simple (Dark)",
            GameTheme::Classic => "Classic",
        })
    }
}

impl Display for MenuTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MenuTheme::Light => "Light",
            MenuTheme::Dark => "Dark",
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: Theme {
                game_theme: GameTheme::SimpleLight,
                menu_theme: MenuTheme::Light,
            },
            scale_factor: 1.0,
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

    pub(crate) fn get_scale_factor(&self) -> f64 {
        self.scale_factor
    }
    pub(crate) fn update_scale_factor(&mut self, scale: f64) {
        self.scale_factor = scale;
    }
}
