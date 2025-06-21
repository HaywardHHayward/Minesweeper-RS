use std::{collections::HashMap, future::Future, pin::Pin};

use iced::{Element, Subscription, Task};

pub mod game;
pub mod main_menu;
pub mod settings;

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
}

impl Default for Application {
    fn default() -> Self {
        let mut screens = HashMap::with_capacity(3);
        screens.insert(ScreenType::MainMenu, Screen::MainMenu(main_menu::MainMenu));
        Application {
            current_screen: ScreenType::MainMenu,
            screens,
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
            Message::ScreenAction(screen_message) => self
                .screens
                .get_mut(&self.current_screen)
                .unwrap_or_else(|| panic!("current_screen {:?} not found", self.current_screen))
                .update(screen_message),
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        self.screens
            .get(&self.current_screen)
            .unwrap_or_else(|| panic!("current_screen {:?} not found", self.current_screen))
            .view()
            .map(Message::ScreenAction)
    }
    fn subscription(&self) -> Subscription<Self::Message> {
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
