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

#[derive(Debug, Clone)]
pub enum Action {
    OpenCell(u8, u8),
    ToggleFlag(u8, u8),
    OpenUnflagged(u8, u8),
}

impl ScreenTrait for Game {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<AppMessage> {
        match message {
            Action::OpenCell(x, y) => {
                self.board.open_cell(x, y);
            }
            Action::ToggleFlag(x, y) => {
                self.board.toggle_flag(x, y);
            }
            Action::OpenUnflagged(x, y) => {
                self.board.open_unflagged(x, y);
            }
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Self::Message> {
        let mut board_column = GuiWidget::Column::with_capacity(self.board.get_height() as usize);
        for y in 0..self.board.get_height() {
            let mut row = GuiWidget::Row::with_capacity(self.board.get_width() as usize);
            for x in 0..self.board.get_width() {
                row = row.push(self.cell(x, y));
            }
            row = row.spacing(4);
            board_column = board_column.push(row);
        }
        let board_view = GuiWidget::container(board_column)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);
        board_view.into()
    }
}

impl Game {
    pub(crate) fn cell(&self, x: u8, y: u8) -> Element<'_, Action> {
        fn cell_view(cell: &crate::core::cell::Cell) -> GuiWidget::Text<'_> {
            // Will become more complex with themes and introducing images instead of purely
            // text, but for now is simple
            if cell.is_open() {
                if cell.is_mine() {
                    return GuiWidget::text("*");
                }
                let adjacent_mines = cell.adjacent_mines().unwrap_or(0);
                return if adjacent_mines == 0 {
                    GuiWidget::text(" ")
                } else {
                    GuiWidget::text(adjacent_mines.to_string())
                };
            }
            if cell.is_flagged() {
                GuiWidget::text("F")
            } else {
                GuiWidget::text("O")
            }
        }
        let cell = self.board.get_cell(x, y).expect("Cell should exist");
        let cell_view = cell_view(cell).font(iced::font::Font::MONOSPACE);
        GuiWidget::mouse_area(cell_view)
            .on_press(Action::OpenCell(x, y))
            .on_right_press(Action::ToggleFlag(x, y))
            .on_middle_press(Action::OpenUnflagged(x, y))
            .into()
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
