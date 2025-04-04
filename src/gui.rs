use std::collections::HashMap;

use iced::{Element, Task};

pub mod main_menu;

#[derive(PartialEq, Eq, Hash)]
enum Screen {
    MainMenu,
}

enum ScreenState {
    MainMenu(main_menu::MainMenu),
}

impl ScreenState {
    fn view(&self) -> Element<Message> {
        match self {
            ScreenState::MainMenu(main_menu) => main_menu.view(),
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
        Self {
            current_screen: Screen::MainMenu,
            screens,
        }
    }
}

#[derive(Debug)]
pub enum Message {
    MainMenu(main_menu::Message),
}

impl Application {
    fn update(&mut self, message: Message) -> Task<Message> {
        let screen = self
            .screens
            .get_mut(&self.current_screen)
            .expect("Current screen should always exist");
        match (screen, message) {
            (ScreenState::MainMenu(main_menu), Message::MainMenu(msg)) => main_menu.update(msg),
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
