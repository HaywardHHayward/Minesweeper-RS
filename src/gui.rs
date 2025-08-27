use std::sync::{Arc, RwLock};

use iced::{Element, Subscription, Task};
use screens::Message;

pub mod assets;
pub mod config;
pub mod screens;

pub fn update(state: &mut Application, message: Message) -> Task<Message> {
    state.update(message).unwrap_or(Task::none())
}

pub fn view(state: &Application) -> Element<'_, Message> {
    state.view()
}

pub fn subscription(state: &Application) -> Subscription<Message> {
    state.subscription().unwrap_or(Subscription::none())
}

pub fn theme(state: &Application) -> iced::Theme {
    state.theme()
}

pub fn scale_factor(state: &Application) -> f64 {
    state.config.read().unwrap().scale_factor
}

pub trait Screen: std::fmt::Debug + Send + Sync {
    fn update(&mut self, message: Message) -> Option<Task<Message>> {
        let _ = message;
        None
    }
    fn view(&self) -> Element<'_, Message> {
        iced::widget::text!("Unfinished screen, current state: {self:?}").into()
    }
    fn subscription(&self) -> Option<Subscription<Message>> {
        None
    }
}

pub type ArcLock<T> = Arc<RwLock<T>>;

#[derive(Clone)]
pub enum AppMessage {
    ChangeScreen(Arc<Box<dyn Screen>>),
    CloseApp,
}

impl std::fmt::Debug for AppMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppMessage::ChangeScreen(_) => write!(f, "ChangeScreen"),
            AppMessage::CloseApp => write!(f, "CloseApp"),
        }
    }
}

#[derive(Debug)]
pub struct Application {
    screen: Box<dyn Screen>,
    config: ArcLock<config::Config>,
}

impl Screen for Application {
    fn update(&mut self, message: Message) -> Option<Task<Message>> {
        let Message::App(message) = message else {
            return self.screen.update(message);
        };
        match message {
            AppMessage::ChangeScreen(builder) => {
                self.screen = Arc::into_inner(builder).unwrap();
                None
            }
            AppMessage::CloseApp => {
                self.clear_cache().unwrap_or_else(|e| {
                    eprintln!("Failed to clear cache: {e}");
                });
                let config = self.config.read().unwrap();
                let config_path = Application::app_dirs().config_dir().join("config.yaml");
                config.save(&config_path);
                Some(iced::exit())
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        self.screen.view()
    }
    fn subscription(&self) -> Option<Subscription<Message>> {
        let close_subscription =
            iced::window::close_requests().map(|_| Message::App(AppMessage::CloseApp));
        if let Some(sub_subscription) = self.screen.subscription() {
            Some(Subscription::batch([close_subscription, sub_subscription]))
        } else {
            Some(close_subscription)
        }
    }
}

impl Application {
    pub fn app_dirs() -> directories::ProjectDirs {
        directories::ProjectDirs::from("", "HaywardHHayward", "Minesweeper")
            .expect("Failed to get project directories")
    }
    pub fn theme(&self) -> iced::Theme {
        match &self.config.read().unwrap().theme.menu_theme {
            config::MenuTheme::Light => iced::Theme::Light,
            config::MenuTheme::Dark => iced::Theme::Dark,
        }
    }
    pub fn clear_cache(&mut self) -> Result<(), std::io::Error> {
        let cache_dir = Self::app_dirs().cache_dir().to_path_buf();
        if cache_dir.exists() {
            std::fs::remove_dir_all(&cache_dir)?;
        }
        Ok(())
    }
    pub fn create() -> Self {
        let config_dir = Application::app_dirs().config_dir().to_path_buf();
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        }
        let config_path = config_dir.join("config.yaml");
        let config = if config_path.exists() {
            config::Config::load(&config_path).unwrap_or_else(|_| {
                // Placeholder for proper error handling, log it for now and use default config,
                // making sure to save it so that a proper config file exists next time
                eprintln!("Failed to load config, using default settings");
                let config = config::Config::default();
                config.save(&config_path);
                config
            })
        } else {
            // If the config file does not exist, create a default config and save it
            let config = config::Config::default();
            config.save(&config_path);
            config
        };
        let config = Arc::new(RwLock::new(config));
        Application {
            screen: Box::new(screens::main_menu::MainMenu::build(config.clone())),
            config,
        }
    }
}
