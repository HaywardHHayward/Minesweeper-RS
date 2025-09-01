use std::{
    num::{NonZeroU8, NonZeroU16},
    sync::Arc,
    time::Instant,
};

use iced::{Element, Subscription, Task, widget as GuiWidget, widget::svg as GuiSvg};

use super::{AppMessage, MainMenu, Message as SuperMessage};
use crate::{ArcLock, Board, BoardState, Cell, Config, GameTheme, Screen};

#[derive(Debug, Clone)]
pub enum Message {
    OpenCell(u8, u8),
    ToggleFlag(u8, u8),
    ChordCell(u8, u8),
    ResetGame,
    TimeUpdate(Instant),
    Back,
}

#[derive(Debug)]
pub struct Game {
    config: ArcLock<Config>,
    board: Board,
    start_time: Instant,
    current_time: Instant,
}

impl Game {
    pub fn build(config: ArcLock<Config>, board: Board) -> Self {
        let game_start = Instant::now();
        Self {
            config,
            board,
            start_time: game_start,
            current_time: game_start,
        }
    }
}

impl Screen for Game {
    fn update(&mut self, message: SuperMessage) -> Option<Task<SuperMessage>> {
        let SuperMessage::Game(message) = message else {
            return None;
        };
        let config = self.config.clone();
        match message {
            Message::OpenCell(x, y) => {
                self.board.open_cell(x, y);
                None
            }
            Message::ToggleFlag(x, y) => {
                self.board.toggle_flag(x, y);
                None
            }
            Message::ChordCell(x, y) => {
                self.board.chord_cell(x, y);
                None
            }
            Message::ResetGame => {
                let (width, height, mines) = (
                    self.board.get_width(),
                    self.board.get_height(),
                    self.board.get_mine_count(),
                );
                let new_board = unsafe {
                    // SAFETY: Rows, columns, and mines are guaranteed to be non-zero since they
                    // were used to create the current board.
                    Board::create_custom(
                        NonZeroU8::new_unchecked(width),
                        NonZeroU8::new_unchecked(height),
                        NonZeroU16::new_unchecked(mines),
                    )
                    .unwrap()
                };
                let new_start = Instant::now();
                self.start_time = new_start;
                self.current_time = new_start;
                self.board = new_board;
                None
            }
            Message::TimeUpdate(time) => {
                self.current_time = time;
                None
            }
            Message::Back => Some(Task::perform(
                async { MainMenu::build(config) },
                move |item| SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item)))),
            )),
        }
    }
    fn view(&self) -> Element<'_, SuperMessage> {
        let board = GuiWidget::container(self.board())
            .style(GuiWidget::container::bordered_box)
            .padding(10);
        let top_bar = GuiWidget::container(self.top_bar())
            .style(GuiWidget::container::bordered_box)
            .padding(10);
        let mut game_content = GuiWidget::column![top_bar, board]
            .spacing(10)
            .align_x(iced::Center);
        if let Some(end_content) = self.end_of_screen() {
            game_content = game_content.push(end_content);
        }
        GuiWidget::center(game_content).into()
    }
    fn subscription(&self) -> Option<Subscription<SuperMessage>> {
        match self.board.get_state() {
            BoardState::InProgress => Some(
                iced::time::every(std::time::Duration::from_secs(1))
                    .map(Message::TimeUpdate)
                    .map(SuperMessage::Game),
            ),
            _ => None,
        }
    }
}

macro_rules! impl_game_image {
    ($([$static_name:ident, $function_name:ident]),*) => {
        impl Game {
            $(
                fn $function_name(&self) -> Element<'_, SuperMessage> {
                    match self.config.read().unwrap().game_theme {
                        GameTheme::SimpleLight => GuiSvg::Svg::new(GuiSvg::Handle::from_memory(
                            crate::gui::assets::simple_light::$static_name.as_slice(),
                        )).into(),
                        GameTheme::SimpleDark => GuiSvg::Svg::new(GuiSvg::Handle::from_memory(
                            crate::gui::assets::simple_dark::$static_name.as_slice(),
                        )).into(),
                        #[cfg(feature = "non-free")]
                        GameTheme::Classic => GuiSvg::Svg::new(GuiSvg::Handle::from_memory(
                            crate::gui::assets::classic::$static_name.as_slice(),
                        )).into()
                    }
                }
            )*
        }
    }
}

impl_game_image!(
    [UNOPENED_CELL, unopened_cell],
    [OPENED_CELL, opened_cell],
    [MINE, mine],
    [FLAG, flag],
    [INCORRECT_FLAG, incorrect_flag],
    [EXPLODED_MINE, exploded_mine]
);

impl Game {
    pub fn reset_button(&self) -> Element<'_, SuperMessage> {
        // TODO: Change icon based on game state (in progress, won, lost), as well as
        // make it based on theme.
        GuiWidget::button(GuiWidget::text("Reset"))
            .on_press(SuperMessage::Game(Message::ResetGame))
            .style(GuiWidget::button::danger)
            .into()
    }
    pub fn top_bar(&self) -> Element<'_, SuperMessage> {
        let remaining_mine_count = self.board.get_remaining_mine_count();
        let mine_count_text = GuiWidget::text(format!("{remaining_mine_count:03}"));

        let reset_button = self.reset_button();

        let elapsed_time = self.current_time.duration_since(self.start_time).as_secs();
        let time_text = GuiWidget::text!("{elapsed_time:03}");

        let content = GuiWidget::row![
            GuiWidget::container(mine_count_text).width(iced::Fill),
            GuiWidget::center_x(reset_button),
            GuiWidget::right(time_text)
        ];
        content
            .width((self.board.get_width() as usize * 16) as f32)
            .into()
    }
    pub fn end_of_screen(&self) -> Option<Element<'_, SuperMessage>> {
        let text = GuiWidget::text(match self.board.get_state() {
            BoardState::Won => "You found all the mines. You win!",
            BoardState::Lost => "You hit a mine! You lose!",
            BoardState::InProgress => "",
        });
        let menu_theme = &self.config.read().unwrap().menu_theme;
        let return_button = menu_theme
            .button("Return to main menu", crate::MenuButtonStyle::Secondary)
            .on_press(SuperMessage::Game(Message::Back));
        let content = GuiWidget::column![text, return_button]
            .align_x(iced::Center)
            .into();
        Some(content)
    }
    pub fn board(&self) -> impl Into<Element<'_, SuperMessage>> {
        let mut board_content = GuiWidget::Grid::with_capacity(
            self.board.get_width() as usize * self.board.get_height() as usize,
        )
        .columns(self.board.get_width() as usize)
        .width(self.board.get_width() as f32 * 16.0);
        for y in 0..self.board.get_height() {
            for x in 0..self.board.get_width() {
                board_content = board_content.push(self.cell(x, y));
            }
        }
        board_content
    }
    pub fn cell(&self, x: u8, y: u8) -> Element<'_, SuperMessage> {
        let cell = self.board.get_cell(x, y).unwrap();
        let content = self.cell_content(cell);
        let is_playing = matches!(self.board.get_state(), BoardState::InProgress);
        if is_playing {
            GuiWidget::mouse_area(content)
                .on_press(SuperMessage::Game(Message::OpenCell(x, y)))
                .on_right_press(SuperMessage::Game(Message::ToggleFlag(x, y)))
                .on_middle_press(SuperMessage::Game(Message::ChordCell(x, y)))
                .into()
        } else {
            content
        }
    }
    pub fn cell_content(&self, cell: &Cell) -> Element<'_, SuperMessage> {
        let content = if cell.is_open() {
            if cell.is_mine() {
                self.exploded_mine()
            } else {
                let adjacent_mines = cell.adjacent_mines().unwrap();
                if adjacent_mines == 0 {
                    self.opened_cell()
                } else {
                    let mut stack = GuiWidget::Stack::with_capacity(2);
                    stack = stack.push(self.opened_cell());
                    let color = match adjacent_mines {
                        1 => iced::color!(0, 0, 255),
                        2 => iced::color!(0, 127, 0),
                        3 => iced::color!(255, 0, 0),
                        4 => iced::color!(0, 0, 127),
                        5 => iced::color!(127, 0, 0),
                        6 => iced::color!(0, 127, 127),
                        7 => iced::color!(255, 255, 255),
                        8 => iced::color!(127, 127, 127),
                        // SAFETY: The internal enum AdjacentMines (which is what
                        // cell.adjacent_mines converts from) CANNOT
                        // represent values outside 0-8, and we just checked
                        // that adjacent_mines is not 0, so all other values are
                        // unreachable.
                        _ => unsafe { std::hint::unreachable_unchecked() },
                    };
                    let text = GuiWidget::center(
                        GuiWidget::text!("{adjacent_mines}")
                            .size(14)
                            .font(iced::font::Font::MONOSPACE)
                            .color(color),
                    )
                    .width(16)
                    .height(16);
                    stack = stack.push(text);
                    stack.into()
                }
            }
        } else {
            match self.board.get_state() {
                BoardState::InProgress => {
                    if cell.is_flagged() {
                        self.flag()
                    } else {
                        self.unopened_cell()
                    }
                }
                BoardState::Won => {
                    if cell.is_mine() || cell.is_flagged() {
                        self.flag()
                    } else {
                        self.unopened_cell()
                    }
                }
                BoardState::Lost => {
                    if cell.is_mine() && !cell.is_flagged() {
                        self.mine()
                    } else if !cell.is_mine() && cell.is_flagged() {
                        self.incorrect_flag()
                    } else if cell.is_flagged() {
                        self.flag()
                    } else {
                        self.unopened_cell()
                    }
                }
            }
        };
        GuiWidget::center(content).width(16).height(16).into()
    }
}
