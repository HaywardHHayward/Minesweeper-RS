use std::collections::HashMap;

use iced::{Element, Subscription, Task};

mod assets;
mod config;

pub fn update(state: &mut Application, message: PublicMessage) -> Task<PublicMessage> {
    let message = message.0;
    state.update(message).map(PublicMessage)
}

pub fn view(state: &Application) -> Element<'_, PublicMessage> {
    state.view().map(PublicMessage)
}

pub fn subscription(state: &Application) -> Subscription<PublicMessage> {
    state.subscription().map(PublicMessage)
}

pub fn theme(state: &Application) -> iced::Theme {
    state.theme()
}

/// Trait that defines the interface for screens in the application. Based on
/// the Elm architecture.
pub(crate) trait ScreenTrait: std::fmt::Debug {
    /// The type of messages that the screen can handle.
    type Message: std::fmt::Debug;
    /// Updates the screen's state based on the provided message. Returns a task
    /// containing the highest level of message to be processed by the
    /// application.
    ///
    /// Returns the highest level of message to allow for screens to
    /// do application level actions such as changing the screen or
    /// initializing a new screen, but only receives messages that are specific
    /// to the screen, since only the highest level abstraction of the
    /// application should HANDLE application level actions.
    fn update(&mut self, message: Self::Message) -> Task<Message> {
        let _ = message;
        Task::none()
    }
    /// Returns the current declarative view of the screen as an `Element`.
    fn view(&self) -> Element<'_, Self::Message> {
        iced::widget::text!("Unfinished screen, current state: {self:?}").into()
    }
    /// Returns a subscription for the screen, which can be used to handle
    /// passive events such as checking for keypresses, or time based events.
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
    pub(crate) fn app_dirs() -> directories::ProjectDirs {
        directories::ProjectDirs::from("", "HaywardHHayward", "Minesweeper")
            .expect("Failed to get project directories")
    }
    pub(crate) fn theme(&self) -> iced::Theme {
        match self.config.get_menu_theme() {
            config::MenuTheme::Light => iced::Theme::Light,
            config::MenuTheme::Dark => iced::Theme::Dark,
        }
    }
}

impl Default for Application {
    fn default() -> Self {
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
        let mut screens = HashMap::with_capacity(3);
        screens.insert(ScreenType::MainMenu, Screen::MainMenu(main_menu::MainMenu));
        screens.insert(
            ScreenType::SettingsScreen,
            Screen::SettingsScreen(settings_screen::SettingsScreen::new(config.clone())),
        );
        screens.insert(ScreenType::About, Screen::About(about::About));
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
                initializer_fn,
            } => {
                let screen = initializer_fn(());
                self.screens.insert(screen_type, screen);
                self.current_screen = screen_type;
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
            Message::ReadConfig => {
                self.config =
                    config::Config::load(&Self::app_dirs().config_dir().join("config.yaml"))
                        .unwrap();
                Task::none()
            }
            Message::SendConfig(callback) => {
                let config = self.config.clone();
                Task::done(callback(config))
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

// Automatically adds new screen modules, new variants to ScreenMessage and
// ScreenType, and automatically adds the new screen types to Screen::update,
// Screen::view, and Screen::subscription
macro_rules! create_screens {
    ($([$snake_case:ident, $pascal_case:ident]),*) => {
        $(pub(crate) mod $snake_case;)*

        #[derive(Debug)]
        pub(crate) enum ScreenMessage {
            $($pascal_case($snake_case::Action),)*
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub(crate) enum ScreenType {
            $($pascal_case,)*
        }

        #[derive(Debug)]
        pub(crate) enum Screen {
            $($pascal_case($snake_case::$pascal_case),)*
        }

        impl ScreenTrait for Screen {
            type Message = ScreenMessage;
            fn update(&mut self, message: Self::Message) -> Task<Message> {
                match (self, message) {
                    $((Self::$pascal_case($snake_case), Self::Message::$pascal_case(action)) => $snake_case.update(action),)*
                    _ => Task::none()
                }
            }
            fn view(&self) -> Element<'_, Self::Message> {
                match self {
                    $(Self::$pascal_case($snake_case) => $snake_case.view().map(Self::Message::$pascal_case),)*
                }
            }
            fn subscription(&self) -> Subscription<Self::Message> {
                match self {
                    $(Self::$pascal_case($snake_case) => $snake_case.subscription().map(Self::Message::$pascal_case),)*
                }
            }
        }
    };
}

create_screens! {
    [about, About],
    [game, Game],
    [game_selection, GameSelection],
    [main_menu, MainMenu],
    [settings_screen, SettingsScreen]
}

#[derive(Debug)]
pub struct PublicMessage(Message);

type Callback<Input, Output> = Box<dyn FnOnce(Input) -> Output + Send + Sync>;

pub(crate) enum Message {
    InitializeScreen {
        screen_type: ScreenType,
        initializer_fn: Callback<(), Screen>,
    },
    ChangeScreen(ScreenType),
    ScreenAction(ScreenMessage),
    ReadConfig,
    SendConfig(Callback<config::Config, Message>),
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::InitializeScreen {
                screen_type,
                initializer_fn: _,
            } => {
                write!(f, "InitializeScreen({screen_type:?}, .. )")
            }
            Message::ChangeScreen(screen_type) => write!(f, "ChangeScreen({screen_type:?})"),
            Message::ScreenAction(action) => write!(f, "ScreenAction({action:?})"),
            Message::ReadConfig => write!(f, "ReadConfig"),
            Message::SendConfig(_) => write!(f, "SendConfig(..)"),
        }
    }
}
