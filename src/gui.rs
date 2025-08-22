use iced::{Element, Subscription, Task};
use screens::Message;

pub(crate) mod assets;
pub(crate) mod config;
pub(crate) mod screens;
pub(crate) mod theme;

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
    state.config.borrow().get_scale_factor()
}

pub trait Screen: std::fmt::Debug {
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

pub(crate) type RcCell<T> = std::rc::Rc<std::cell::RefCell<T>>;

pub enum AppMessage {
    ChangeScreen(Box<dyn FnOnce() -> Box<dyn Screen> + Send + Sync>),
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
    config: RcCell<config::Config>,
}

impl Screen for Application {
    fn update(&mut self, message: Message) -> Option<Task<Message>> {
        self.screen.update(message)
    }
    fn view(&self) -> Element<'_, Message> {
        self.screen.view()
    }
    fn subscription(&self) -> Option<Subscription<Message>> {
        self.screen.subscription()
    }
}

impl Application {
    pub(crate) fn app_dirs() -> directories::ProjectDirs {
        directories::ProjectDirs::from("", "HaywardHHayward", "Minesweeper")
            .expect("Failed to get project directories")
    }
    pub(crate) fn theme(&self) -> iced::Theme {
        match self.config.borrow().get_menu_theme() {
            config::MenuTheme::Light => iced::Theme::Light,
            config::MenuTheme::Dark => iced::Theme::Dark,
        }
    }
    pub(crate) fn clear_cache(&mut self) -> Result<(), std::io::Error> {
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
        let config = std::rc::Rc::new(std::cell::RefCell::new(config));
        Application {
            screen: Box::new(screens::main_menu::MainMenu::build(config.clone())),
            config,
        }
    }
}
