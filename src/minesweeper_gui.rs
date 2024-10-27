use iced::{widget::*, *};

use crate::minesweeper_gui::game_screen::*;

mod game_screen;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenTile(u8, u8),
    FlagTile(u8, u8),
    SafeOpenTile(u8, u8),
    ChangeScreen(ScreenChoices),
}
trait Screen {
    fn update(&mut self, message: Message) -> Task<Message>;
    fn view(&self) -> Element<Message>;
}

#[derive(Debug, Clone, Copy)]
pub enum ScreenChoices {
    Game = 0,
}

pub struct MinesweeperApp {
    current_screen: ScreenChoices,
    screens: Vec<Box<dyn Screen>>,
}

impl Default for MinesweeperApp {
    fn default() -> Self {
        let screens: Vec<Box<dyn Screen>> = vec![Box::new(GameScreen::new(10, 10, 10))];
        Self {
            current_screen: ScreenChoices::Game,
            screens,
        }
    }
}

impl MinesweeperApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        if let Message::ChangeScreen(change) = message {
            self.current_screen = change;
        }
        self.current_screen_mut().update(message)
    }
    pub fn view(&self) -> Element<Message> {
        self.current_screen().view()
    }
    fn current_screen(&self) -> &dyn Screen {
        match self.current_screen {
            ScreenChoices::Game => self.screens[0].as_ref(),
        }
    }
    fn current_screen_mut(&mut self) -> &mut dyn Screen {
        match self.current_screen {
            ScreenChoices::Game => self.screens[0].as_mut(),
        }
    }
}
