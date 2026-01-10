use std::{
    num::{NonZeroU8, NonZeroU16},
    ops::DerefMut,
    sync::{Arc, RwLock},
};

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

pub fn scale_factor(state: &Application) -> f32 {
    state.scale_factor()
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
    NextFocus,
    PreviousFocus,
    Error(Arc<anyhow::Error>),
    CleanConfig(Box<Message>),
}

impl std::fmt::Debug for AppMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppMessage::ChangeScreen(_) => write!(f, "ChangeScreen"),
            AppMessage::CloseApp => write!(f, "CloseApp"),
            AppMessage::NextFocus => write!(f, "NextFocus"),
            AppMessage::PreviousFocus => write!(f, "PreviousFocus"),
            AppMessage::Error(error) => {
                write!(f, "Error: {error}")
            }
            AppMessage::CleanConfig(message) => {
                write!(f, "CleanConfig: {message:?}")
            }
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
            AppMessage::ChangeScreen(builder) => match Arc::into_inner(builder) {
                Some(new_screen) => {
                    self.screen = new_screen;
                    None
                }
                None => Some(
                    Task::done(anyhow::Error::msg("Failed to receive screen"))
                        .map(Arc::from)
                        .map(AppMessage::Error)
                        .map(Message::App),
                ),
            },
            AppMessage::CloseApp => {
                self.clear_cache().unwrap_or_else(|e| {
                    eprintln!("Failed to clear cache: {e}");
                });
                let Ok(config) = self.config.read() else {
                    return Some(
                        Task::done(AppMessage::CloseApp)
                            .map(Message::App)
                            .map(Box::new)
                            .map(AppMessage::CleanConfig)
                            .map(Message::App),
                    );
                };
                let config_dir_path = Application::app_dirs().config_dir().to_path_buf();
                if !config_dir_path.exists()
                    && let Err(error) = std::fs::create_dir_all(&config_dir_path)
                {
                    return Some(
                        Task::done(error.into())
                            .map(Arc::new)
                            .map(AppMessage::Error)
                            .map(Message::App),
                    );
                }
                let config_path = config_dir_path.join("config.yaml");
                config.save(&config_path);
                Some(iced::exit())
            }
            AppMessage::NextFocus => Some(iced::widget::operation::focus_next()),
            AppMessage::PreviousFocus => Some(iced::widget::operation::focus_previous()),
            AppMessage::Error(error) => {
                let error_message = rfd::AsyncMessageDialog::new()
                    .set_level(rfd::MessageLevel::Error).set_title("Error has occurred in Minesweeper")
                    .set_description(format!("An error has occurred in Minesweeper. Program will close upon selecting 'Ok'. Context: {error}"))
                    .set_buttons(rfd::MessageButtons::Ok).show();
                Some(Task::perform(error_message, |_| {
                    Message::App(AppMessage::CloseApp)
                }))
            }
            AppMessage::CleanConfig(message) => {
                let rw_config = self.config.clone();
                if let Err(mut poisoned_config) = self.config.write() {
                    *poisoned_config.get_mut().deref_mut() = config::Config::default();
                }
                rw_config.clear_poison();
                Some(Task::done(*message))
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        self.screen.view()
    }
    fn subscription(&self) -> Option<Subscription<Message>> {
        let close_subscription =
            iced::window::close_requests().map(|_| Message::App(AppMessage::CloseApp));
        let change_focus_subscription = iced::keyboard::listen().filter_map(|event| {
            let iced::keyboard::Event::KeyPressed { key, modifiers, .. } = event else {
                return None;
            };
            match key {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) => {
                    if modifiers.is_empty() {
                        Some(Message::App(AppMessage::NextFocus))
                    } else if modifiers.contains(iced::keyboard::Modifiers::SHIFT) {
                        Some(Message::App(AppMessage::PreviousFocus))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        });
        let app_subscription = Subscription::batch([close_subscription, change_focus_subscription]);
        if let Some(screen_subscription) = self.screen.subscription() {
            Some(Subscription::batch([app_subscription, screen_subscription]))
        } else {
            Some(app_subscription)
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::create()
    }
}

impl Application {
    pub fn app_dirs() -> directories::ProjectDirs {
        directories::ProjectDirs::from("", "HaywardHHayward", "Minesweeper")
            .expect("Failed to get project directories")
    }
    pub fn theme(&self) -> iced::Theme {
        self.config
            .read()
            .map(|config| config.menu_theme.theme())
            .unwrap_or_else(|_| config::MenuTheme::Light.theme())
    }
    pub fn scale_factor(&self) -> f32 {
        self.config
            .read()
            .map(|config| config.scale_factor)
            .unwrap_or(1.0)
    }
    pub fn clear_cache(&mut self) -> Result<(), std::io::Error> {
        let cache_dir = Self::app_dirs().cache_dir().to_path_buf();
        if cache_dir.exists() {
            std::fs::remove_dir_all(&cache_dir)?;
        }
        Ok(())
    }
    fn initialize_config() -> config::Config {
        let config_path = Application::app_dirs()
            .config_dir()
            .join("config.yaml")
            .to_path_buf();
        if config_path.exists() {
            config::Config::load(&config_path).unwrap_or_else(|_| {
                // Placeholder for proper error handling, log it for now and use default config,
                // making sure to save it so that a proper config file exists next time
                eprintln!("Failed to load config, using default settings");
                let config = config::Config::default();
                config.save(&config_path);
                config
            })
        } else {
            config::Config::default()
        }
    }
    pub fn create() -> Self {
        let config = Arc::new(RwLock::new(Self::initialize_config()));
        Application {
            screen: Box::new(screens::main_menu::MainMenu::build(config.clone())),
            config,
        }
    }
    pub fn create_beginner() -> Self {
        let config = Arc::new(RwLock::new(Self::initialize_config()));
        let board = crate::Board::create_beginner();
        Application {
            screen: Box::new(screens::game::Game::build(config.clone(), board)),
            config,
        }
    }
    pub fn create_intermediate() -> Self {
        let config = Arc::new(RwLock::new(Self::initialize_config()));
        let board = crate::Board::create_intermediate();
        Application {
            screen: Box::new(screens::game::Game::build(config.clone(), board)),
            config,
        }
    }
    pub fn create_expert() -> Self {
        let config = Arc::new(RwLock::new(Self::initialize_config()));
        let board = crate::Board::create_expert();
        Application {
            screen: Box::new(screens::game::Game::build(config.clone(), board)),
            config,
        }
    }
    pub fn create_custom(width: NonZeroU8, height: NonZeroU8, mines: NonZeroU16) -> Self {
        let config = Arc::new(RwLock::new(Self::initialize_config()));
        // TODO: Validate custom board parameters more gracefully
        if mines.get() >= (width.get() as u16 * height.get() as u16) {
            panic!("Mines must be less than width * height");
        }
        let board = crate::Board::create_custom(width, height, mines).unwrap();
        Application {
            screen: Box::new(screens::game::Game::build(config.clone(), board)),
            config,
        }
    }
}
