use iced::{widget::*, *};

use crate::minesweeper_app::game_screen::*;

mod game_screen;

pub struct MinesweeperApp {
    current_screen: Box<dyn Screen>,
}

impl Default for MinesweeperApp {
    fn default() -> Self {
        Self {
            current_screen: Box::new(GameScreen::new(10, 10, 10)),
        }
    }
}

enum AppScreen {
    GameScreen(GameScreen),
}

trait Screen {
    fn update(&mut self, message: Message) -> Task<Message>;
    fn view(&self) -> Element<Message>;
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenTile(u8, u8),
    FlagTile(u8, u8),
    SafeOpenTile(u8, u8),
}

impl MinesweeperApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        self.current_screen.update(message)
    }
    pub fn view(&self) -> Element<Message> {
        self.current_screen.view()
    }
}
