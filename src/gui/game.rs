use iced::{Element, Task, widget as GuiWidget, widget::image as GuiImage};

use crate::{
    core::board::Board,
    gui::{Message as AppMessage, ScreenMessage, ScreenTrait, ScreenType},
};

#[derive(Debug)]
pub struct Game {
    board: Board,
}

#[derive(Debug, Clone)]
pub enum Action {
    OpenCell(u8, u8),
    ToggleFlag(u8, u8),
    ChordCell(u8, u8),
    CheckGameStatus,
}

impl Game {
    pub fn new(board: Board) -> Self {
        Game { board }
    }
}

impl ScreenTrait for Game {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<AppMessage> {
        match message {
            Action::OpenCell(x, y) => {
                self.board.open_cell(x, y);
                Task::done(AppMessage::ScreenAction(ScreenMessage::Game(
                    Action::CheckGameStatus,
                )))
            }
            Action::ToggleFlag(x, y) => {
                self.board.toggle_flag(x, y);
                Task::none()
            }
            Action::ChordCell(x, y) => {
                self.board.chord_cell(x, y);
                Task::done(AppMessage::ScreenAction(ScreenMessage::Game(
                    Action::CheckGameStatus,
                )))
            }
            Action::CheckGameStatus => {
                let status = self.board.get_state();
                match status {
                    crate::core::board::BoardState::InProgress => Task::none(),
                    crate::core::board::BoardState::Won => {
                        Task::done(AppMessage::ChangeScreen(ScreenType::MainMenu))
                    }
                    crate::core::board::BoardState::Lost => {
                        Task::done(AppMessage::ChangeScreen(ScreenType::MainMenu))
                    }
                }
            }
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        let mut board_column = GuiWidget::Column::with_capacity(self.board.get_height() as usize);
        for y in 0..self.board.get_height() {
            let mut row = GuiWidget::Row::with_capacity(self.board.get_width() as usize);
            for x in 0..self.board.get_width() {
                row = row.push(self.cell(x, y));
            }
            board_column = board_column.push(row);
        }
        let board_view = GuiWidget::container(board_column);
        board_view.center(iced::Length::Fill).into()
    }
}

mod image_default {
    pub(super) static OPENED_CELL: &[u8] = include_bytes!("../../assets/default/OpenedCell.png");
    pub(super) static UNOPENED_CELL: &[u8] =
        include_bytes!("../../assets/default/UnopenedCell.png");
    pub(super) static MINE: &[u8] = include_bytes!("../../assets/default/Mine.png");
    pub(super) static FLAG: &[u8] = include_bytes!("../../assets/default/Flag.png");
}

impl Game {
    fn cell_view(cell: &crate::core::cell::Cell) -> Element<'_, Action> {
        #[inline]
        fn cell_container<'a>(element: impl Into<Element<'a, Action>>) -> Element<'a, Action> {
            GuiWidget::container(element)
                .width(16)
                .height(16)
                .center(iced::Fill)
                .into()
        }
        let mut stack = GuiWidget::Stack::with_capacity(2).height(16).width(16);
        if cell.is_open() {
            let open_image =
                GuiImage::Image::new(GuiImage::Handle::from_bytes(image_default::OPENED_CELL));
            stack = stack.push(open_image);
            if cell.is_mine() {
                let mine_image = cell_container(GuiImage::Image::new(
                    GuiImage::Handle::from_bytes(image_default::MINE),
                ));
                stack = stack.push(mine_image);
            } else if let Some(adjacent_mines) = cell.adjacent_mines()
                && adjacent_mines > 0
            {
                let color = match adjacent_mines {
                    1 => iced::color!(0, 0, 255),
                    2 => iced::color!(0, 127, 0),
                    3 => iced::color!(255, 0, 0),
                    4 => iced::color!(0, 0, 127),
                    5 => iced::color!(127, 0, 0),
                    6 => iced::color!(0, 127, 127),
                    7 => iced::color!(255, 255, 255),
                    8 => iced::color!(127, 127, 127),
                    _ => std::unreachable!(),
                };
                let text = GuiWidget::text!("{adjacent_mines}")
                    .font(iced::font::Font::MONOSPACE)
                    .size(14)
                    .color(color);
                stack = stack.push(cell_container(text));
            }
        } else {
            let unopened_image =
                GuiImage::Image::new(GuiImage::Handle::from_bytes(image_default::UNOPENED_CELL));
            stack = stack.push(unopened_image);
            if cell.is_flagged() {
                let flag_image = cell_container(GuiImage::Image::new(
                    GuiImage::Handle::from_bytes(image_default::FLAG),
                ));
                stack = stack.push(flag_image);
            }
        }
        stack.width(16).height(16).into()
    }
    fn cell(&self, x: u8, y: u8) -> Element<'_, Action> {
        let cell = self.board.get_cell(x, y).expect("Cell should exist");
        let cell_view = Game::cell_view(cell);
        GuiWidget::mouse_area(cell_view)
            .on_press(Action::OpenCell(x, y))
            .on_right_press(Action::ToggleFlag(x, y))
            .on_middle_press(Action::ChordCell(x, y))
            .into()
    }
}
