use std::{fmt::Display, fs::File, path::Path};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub theme: Theme,
    pub scale_factor: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Theme {
    pub game_theme: GameTheme,
    pub menu_theme: MenuTheme,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub enum GameTheme {
    SimpleLight,
    SimpleDark,
    #[cfg(feature = "non-free")]
    Classic,
}

impl GameTheme {
    pub const ALL: &'static [GameTheme] = &[
        GameTheme::SimpleLight,
        GameTheme::SimpleDark,
        #[cfg(feature = "non-free")]
        GameTheme::Classic,
    ];
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub enum MenuTheme {
    Light,
    Dark,
}

impl MenuTheme {
    pub const ALL: &'static [MenuTheme] = &[MenuTheme::Light, MenuTheme::Dark];
}

impl Display for GameTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GameTheme::SimpleLight => "Simple (Light)",
            GameTheme::SimpleDark => "Simple (Dark)",
            #[cfg(feature = "non-free")]
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
    pub fn save(&self, save_location: &Path) {
        let save_file = File::create(save_location).expect("Failed to create config file");
        serde_yml::to_writer(save_file, &self).expect("Failed to serialize config");
    }

    pub fn load(load_location: &Path) -> Result<Self, serde_yml::Error> {
        let config_file = File::open(load_location).expect("Failed to open config file");
        let config = serde_yml::from_reader(config_file)?;
        Ok(config)
    }
}
