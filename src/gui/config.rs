use std::{fs::File, path::Path};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    theme: Theme,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Theme {
    game_theme: (), // Placeholder for game theme
    menu_theme: (), // Placeholder for menu theme
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: Theme {
                game_theme: (), // Default game theme placeholder
                menu_theme: (), // Default menu theme placeholder
            },
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
