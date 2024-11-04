use iced::{widget::*, *};

use crate::minesweeper_gui::{game_screen::*, main_menu::*};

mod game_screen;
mod main_menu;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Game(GameMessage),
    ChangeScreen(ScreenChoices),
    ActivateTimer { start_time: time::Instant },
    DeactivateTimer,
    QueryingTime(time::Instant),
}

trait Screen {
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }
    fn view(&self) -> Element<Message>;
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ScreenChoices {
    Game,
    StartMenu,
}

enum TimerState {
    Inactive,
    Active,
}

pub struct MinesweeperApp {
    current_screen: ScreenChoices,
    screens: [Option<Box<dyn Screen>>; 2],
    timer_state: TimerState,
}

impl Default for MinesweeperApp {
    fn default() -> Self {
        let screens: [Option<Box<dyn Screen>>; 2] = [
            Some(Box::new(GameScreen::new(10, 10, 10))),
            Some(Box::new(MainMenu::new())),
        ];
        Self {
            current_screen: ScreenChoices::StartMenu,
            screens,
            timer_state: TimerState::Inactive,
        }
    }
}

const GAME_INDEX: usize = 0;
const MAIN_MENU_INDEX: usize = 1;

impl MinesweeperApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeScreen(change) => {
                self.current_screen = change;
                if let ScreenChoices::Game = change {
                    return self.current_screen_mut().update(message).chain(Task::done(
                        Message::ActivateTimer {
                            start_time: time::Instant::now(),
                        },
                    ));
                }
            }
            Message::ActivateTimer { .. } => {
                self.timer_state = TimerState::Active;
            }
            Message::DeactivateTimer => {
                self.timer_state = TimerState::Inactive;
            }
            Message::Game(GameMessage::GameFinished { .. }) => {
                self.screens[GAME_INDEX] = Some(Box::new(GameScreen::new(10, 10, 10)));
            }
            _ => return self.current_screen_mut().update(message),
        }
        self.current_screen_mut().update(message)
    }
    pub fn view(&self) -> Element<Message> {
        self.current_screen().view()
    }
    pub fn subscription(&self) -> Subscription<Message> {
        if let TimerState::Active { .. } = self.timer_state {
            time::every(time::Duration::from_secs(1)).map(Message::QueryingTime)
        } else {
            Subscription::none()
        }
    }
    fn current_screen(&self) -> &dyn Screen {
        match self.current_screen {
            ScreenChoices::Game => self.screens[GAME_INDEX].as_deref().unwrap(),
            ScreenChoices::StartMenu => self.screens[MAIN_MENU_INDEX].as_deref().unwrap(),
        }
    }
    fn current_screen_mut(&mut self) -> &mut dyn Screen {
        match self.current_screen {
            ScreenChoices::Game => self.screens[GAME_INDEX].as_deref_mut().unwrap(),
            ScreenChoices::StartMenu => self.screens[MAIN_MENU_INDEX].as_deref_mut().unwrap(),
        }
    }
}
