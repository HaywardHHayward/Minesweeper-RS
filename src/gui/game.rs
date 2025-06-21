use std::num::{NonZeroU8, NonZeroU16};

use iced::{Element, Task, widget as GuiWidget};

use crate::{
    core::board::Board,
    gui::{Message as AppMessage, ScreenTrait},
};

#[derive(Debug)]
pub struct Game {
    board: Board,
}

pub(crate) enum Options {
    Beginner,
    Intermediate,
    Expert,
    Custom { width: u8, height: u8, mines: u16 },
}

#[derive(Debug)]
pub enum Action {
    OpenCell(u8, u8),
    ToggleFlag(u8, u8),
    SafeOpenCell(u8, u8),
}

impl ScreenTrait for Game {
    type Message = Action;

    fn update(&mut self, _message: Self::Message) -> Task<AppMessage> {
        Task::none()
    }
    fn view(&self) -> Element<Self::Message> {
        GuiWidget::text("Game screen is not implemented yet").into()
    }
}

pub(crate) async fn initialize_game(option: Options) -> Game {
    let board = match option {
        Options::Beginner => Board::create_beginner(),
        Options::Intermediate => Board::create_intermediate(),
        Options::Expert => Board::create_expert(),
        Options::Custom {
            width,
            height,
            mines,
        } => Board::create_custom(
            NonZeroU8::new(width).unwrap(),
            NonZeroU8::new(height).unwrap(),
            NonZeroU16::new(mines).unwrap(),
        )
        .expect("Failed to create custom board"),
    };
    Game { board }
}
