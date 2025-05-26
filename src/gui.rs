use std::collections::HashMap;

use iced::{Element, Task};

pub mod main_menu;
pub mod settings;
#[derive(Debug, Clone, Copy)]
pub enum Message {
    MainMenu(main_menu::Message),
    Settings(settings::Message),
    ChangeScreen(Screen),
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Screen {
    MainMenu,
    Settings,
}

pub enum ScreenState {
    MainMenu(main_menu::MainMenu),
    Settings(settings::Settings),
}

impl ScreenState {
    fn view(&self) -> Element<Message> {
        match self {
            ScreenState::MainMenu(main_menu) => main_menu.view().map(Message::MainMenu),
            ScreenState::Settings(settings) => settings.view().map(Message::Settings),
        }
    }
}

pub struct Application {
    current_screen: Screen,
    screens: HashMap<Screen, ScreenState>,
}

impl Default for Application {
    fn default() -> Self {
        let mut screens = HashMap::with_capacity(1);
        screens.insert(Screen::MainMenu, ScreenState::MainMenu(main_menu::MainMenu));
        screens.insert(Screen::Settings, ScreenState::Settings(settings::Settings));
        Self {
            current_screen: Screen::MainMenu,
            screens,
        }
    }
}

trait ScreenTrait: Into<ScreenState> {
    type Message: std::fmt::Debug + Clone + Copy + Into<Message>;

    fn update(&mut self, message: Self::Message) -> Task<Message>;
    fn view(&self) -> Element<Self::Message>;
}

impl Application {
    fn update(&mut self, message: Message) -> Task<Message> {
        let screen = self
            .screens
            .get_mut(&self.current_screen)
            .expect("Current screen should always exist");
        match (screen, message) {
            (_, Message::ChangeScreen(screen)) => {
                self.current_screen = screen;
                Task::none()
            }
            (ScreenState::MainMenu(main_menu), Message::MainMenu(msg)) => main_menu.update(msg),
            (ScreenState::MainMenu(_), _) => Task::none(),
            (ScreenState::Settings(settings), Message::Settings(msg)) => {
                todo!()
            }
            (ScreenState::Settings(_), _) => Task::none(),
        }
    }
    fn view(&self) -> Element<Message> {
        self.screens
            .get(&self.current_screen)
            .expect("Current screen should always exist")
            .view()
    }
}

pub fn update(app: &mut Application, message: Message) -> Task<Message> {
    app.update(message)
}
pub fn view(app: &Application) -> Element<Message> {
    app.view()
}
