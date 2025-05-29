use std::{future::Future, pin::Pin};
use std::collections::HashMap;
use iced::{Element, Subscription, Task};

pub mod game;
pub mod main_menu;
pub mod settings;

#[derive(Debug)]
pub struct Application {
    current_screen: ScreenType,
    screens: HashMap<ScreenType, Screen>
}

impl Default for Application {
    fn default() -> Self {
        Application {
            current_screen: ScreenType::MainMenu,
            screens: HashMap::with_capacity(3),
        }
    }
}

type Callback<Output> = fn() -> Pin<Box<dyn Future<Output = Output> + Send>>;

#[derive(Debug)]
pub enum AppMessage {
    InitializeScreen(ScreenType, Callback<Screen>),
    InitializedScreen(ScreenType, ()),
    ChangeScreen,
    ScreenAction(ScreenMessage),
}

#[derive(Debug)]
pub enum ScreenMessage {
    // Any unit variants are placeholders for screen-specific actions
    MainMenu(main_menu::Action),
    Settings(settings::Action),
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
    Settings(settings::Settings),
    Game(game::Game),
}

impl Screen {
    pub fn update(&mut self, message: ScreenMessage) -> Task<AppMessage> {
        match (self, message) {
            (Screen::MainMenu(menu), ScreenMessage::MainMenu(action)) => {
                // Handle main menu action
                Task::none()
            }
            (Screen::Settings(settings), ScreenMessage::Settings(action)) => {
                // Handle settings action
                Task::none()
            }
            (Screen::Game(game), ScreenMessage::Game(action)) => {
                // Handle game action
                Task::none()
            }
            _ => Task::none(),
        }
    }
    pub fn view(&self) -> Element<AppMessage> {
        match self {
            Screen::MainMenu(menu) => todo!(),
            Screen::Settings(settings) => todo!(),
            Screen::Game(game) => todo!(),
        }
    }
    pub fn subscription(&self) -> Subscription<AppMessage> {
        match self {
            Screen::MainMenu(_) => Subscription::none(),
            Screen::Settings(_) => Subscription::none(),
            Screen::Game(_) => Subscription::none(),
        }
    }
}

impl Application {
    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::InitializeScreen(screen_type, callback) => {
                Task::perform(callback(), move |_| {
                    AppMessage::InitializedScreen(screen_type, ())
                })
            }
            AppMessage::InitializedScreen(screen_type, _) => {
                // Logic to handle the initialized screen can be added here
                Task::none()
            }
            AppMessage::ChangeScreen => {
                // Logic to change the screen can be added here
                Task::none()
            }
            AppMessage::ScreenAction(screen_message) => {
                // Handle screen-specific actions
                Task::none()
            }
        }
    }
    pub fn view(&self) -> Element<AppMessage> {
        iced::widget::text("Hello world!").into()
    }
    pub fn subscription(&self) -> Subscription<AppMessage> {
        Subscription::none()
    }
}

pub fn update(state: &mut Application, message: AppMessage) -> Task<AppMessage> {
    state.update(message)
}

pub fn view(state: &Application) -> Element<AppMessage> {
    state.view()
}

pub fn subscription(state: &Application) -> iced::Subscription<AppMessage> {
    state.subscription()
}
