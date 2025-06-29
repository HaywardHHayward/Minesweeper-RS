use std::{collections::HashMap, future::Future, pin::Pin};

use iced::{Element, Subscription, Task};

mod config;
pub mod game;
pub mod main_menu;
pub mod settings_screen;

pub fn update(state: &mut Application, message: Message) -> Task<Message> {
    state.update(message)
}

pub fn view(state: &Application) -> Element<'_, Message> {
    state.view()
}

pub fn subscription(state: &Application) -> Subscription<Message> {
    state.subscription()
}

pub(crate) trait ScreenTrait {
    type Message: std::fmt::Debug;
    fn update(&mut self, _message: Self::Message) -> Task<Message> {
        Task::none()
    }
    fn view(&self) -> Element<'_, Self::Message> {
        iced::widget::text("Hello, world!").into()
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}

#[derive(Debug)]
pub struct Application {
    current_screen: ScreenType,
    screens: HashMap<ScreenType, Screen>,
    config: config::Config,
}

impl Application {
    pub fn app_dirs() -> directories::ProjectDirs {
        directories::ProjectDirs::from("", "HaywardHHayward", "Minesweeper")
            .expect("Failed to get project directories")
    }
}

impl Default for Application {
    fn default() -> Self {
        let config_dir = Application::app_dirs().config_dir().to_path_buf();
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        }
        let config_path = config_dir.join("config.yml");
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
        let mut screens = HashMap::with_capacity(3);
        screens.insert(ScreenType::MainMenu, Screen::MainMenu(main_menu::MainMenu));
        screens.insert(
            ScreenType::Settings,
            Screen::Settings(settings_screen::SettingsScreen),
        );
        Application {
            current_screen: ScreenType::MainMenu,
            screens,
            config,
        }
    }
}

impl ScreenTrait for Application {
    type Message = Message;
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::InitializeScreen {
                screen_type,
                initializer_fn: callback,
                change_screen,
            } => Task::perform(callback(), move |screen| Message::InitializedScreen {
                screen_type,
                initialized_screen: screen,
            })
            .chain(if change_screen {
                Task::done(Message::ChangeScreen(screen_type))
            } else {
                Task::none()
            }),
            Message::InitializedScreen {
                screen_type,
                initialized_screen: screen,
            } => {
                // Logic to handle the initialized screen can be added here
                self.screens.insert(screen_type, screen);
                Task::none()
            }
            Message::ChangeScreen(screen_type) => {
                self.current_screen = screen_type;
                Task::none()
            }
            Message::ScreenAction(screen_message) => {
                // Message is a ScreenMessage, so we need to pass it along it to the current
                // screen, for now we panic if the screen is not found, but this
                // should be handled more gracefully
                self.screens
                    .get_mut(&self.current_screen)
                    .unwrap_or_else(|| panic!("current_screen {:?} not found", self.current_screen))
                    .update(screen_message)
            }
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        // Retrieve the current screen and call its view method, for now we panic if the
        // screen is not found, but this should be handled more gracefully
        self.screens
            .get(&self.current_screen)
            .unwrap_or_else(|| panic!("current_screen {:?} not found", self.current_screen))
            .view()
            .map(Message::ScreenAction)
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        // Retrieve the current screen and call its subscription method, for now we
        // panic if the screen is not found, but this should be handled more
        // gracefully
        self.screens
            .get(&self.current_screen)
            .unwrap_or_else(|| panic!("current_screen {:?} not found", self.current_screen))
            .subscription()
            .map(Message::ScreenAction)
    }
}

type Callback<Output> = fn() -> Pin<Box<dyn Future<Output = Output> + Send>>;

#[derive(Debug)]
pub enum Message {
    InitializeScreen {
        screen_type: ScreenType,
        initializer_fn: Callback<Screen>,
        change_screen: bool,
    },
    InitializedScreen {
        screen_type: ScreenType,
        initialized_screen: Screen,
    },
    ChangeScreen(ScreenType),
    ScreenAction(ScreenMessage),
}

#[derive(Debug)]
pub enum ScreenMessage {
    MainMenu(main_menu::Action),
    Settings(settings_screen::Action),
    Game(game::Action),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenType {
    MainMenu,
    Settings,
    Game,
}

#[derive(Debug)]
pub enum Screen {
    MainMenu(main_menu::MainMenu),
    Settings(settings_screen::SettingsScreen),
    Game(game::Game),
}

impl ScreenTrait for Screen {
    type Message = ScreenMessage;
    fn update(&mut self, message: ScreenMessage) -> Task<Message> {
        match (self, message) {
            (Screen::MainMenu(menu), ScreenMessage::MainMenu(action)) => menu.update(action),
            (Screen::Settings(settings), ScreenMessage::Settings(action)) => {
                settings.update(action)
            }
            (Screen::Game(game), ScreenMessage::Game(action)) => game.update(action),
            _ => Task::none(),
        }
    }
    fn view(&self) -> Element<'_, ScreenMessage> {
        match self {
            Screen::MainMenu(menu) => menu.view().map(ScreenMessage::MainMenu),
            Screen::Settings(settings) => settings.view().map(ScreenMessage::Settings),
            Screen::Game(game) => game.view().map(ScreenMessage::Game),
        }
    }
    fn subscription(&self) -> Subscription<ScreenMessage> {
        match self {
            Screen::MainMenu(menu) => menu.subscription().map(ScreenMessage::MainMenu),
            Screen::Settings(settings) => settings.subscription().map(ScreenMessage::Settings),
            Screen::Game(game) => game.subscription().map(ScreenMessage::Game),
        }
    }
}
