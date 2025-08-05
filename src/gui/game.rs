use std::{
    num::{NonZeroU8, NonZeroU16},
    time::Instant,
};

use iced::{Element, Font, Subscription, Task, widget as GuiWidget, widget::svg as GuiSvg};

use crate::{
    core::{board::*, cell::*},
    gui::{Message as AppMessage, ScreenTrait, ScreenType, config::GameTheme},
};

#[derive(Debug)]
pub(crate) struct Game {
    board: Board,
    start_time: Instant,
    current_time: Instant,
    game_theme: GameTheme,
}

#[derive(Debug, Clone)]
pub(crate) enum Action {
    OpenCell(u8, u8),
    ToggleFlag(u8, u8),
    ChordCell(u8, u8),
    ResetGame,
    TimeUpdate(Instant),
    ReturnToMainMenu,
}

impl Game {
    pub(crate) fn new(board: Board, game_theme: GameTheme) -> Self {
        let game_start = Instant::now();
        Self {
            board,
            start_time: game_start,
            current_time: game_start,
            game_theme,
        }
    }
}

impl ScreenTrait for Game {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<AppMessage> {
        match message {
            Self::Message::OpenCell(x, y) => {
                self.board.open_cell(x, y);
                Task::none()
            }
            Self::Message::ToggleFlag(x, y) => {
                self.board.toggle_flag(x, y);
                Task::none()
            }
            Self::Message::ChordCell(x, y) => {
                self.board.chord_cell(x, y);
                Task::none()
            }
            Self::Message::ResetGame => {
                let (rows, columns, mine_count) = (
                    self.board.get_height(),
                    self.board.get_width(),
                    self.board.get_mine_count(),
                );
                let new_board = Board::create_custom(
                    NonZeroU8::new(columns).unwrap(),
                    NonZeroU8::new(rows).unwrap(),
                    NonZeroU16::new(mine_count).unwrap(),
                )
                .unwrap();
                let new_start = Instant::now();
                self.start_time = new_start;
                self.current_time = new_start;
                self.board = new_board;
                Task::none()
            }
            Self::Message::TimeUpdate(time) => {
                self.current_time = time;
                Task::none()
            }
            Self::Message::ReturnToMainMenu => {
                Task::done(AppMessage::ChangeScreen(ScreenType::MainMenu))
            }
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        let board_view = GuiWidget::container(self.board())
            .style(GuiWidget::container::bordered_box)
            .padding(10);
        let top_menu_view = GuiWidget::container(self.top_menu())
            .style(GuiWidget::container::bordered_box)
            .padding(10);
        let mut game_content = GuiWidget::column![top_menu_view, board_view].align_x(iced::Center);
        let extra_content = self.end_of_screen();
        if let Some(extra) = extra_content {
            game_content = game_content.push(extra);
        }
        let content = GuiWidget::center(game_content);
        content.into()
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        match self.board.get_state() {
            BoardState::InProgress => {
                iced::time::every(std::time::Duration::from_secs(1)).map(Self::Message::TimeUpdate)
            }
            BoardState::Won | BoardState::Lost => Subscription::none(),
        }
    }
}

macro_rules! impl_game_image {
    ($([$static_name:ident, $function_name:ident]),*) => {
        impl Game {
            $(
                fn $function_name(&self) -> Element<'_, Action> {
                    match self.game_theme {
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
    fn board(&self) -> impl Into<Element<'_, Action>> {
        let mut board_view = GuiWidget::Grid::with_capacity(
            self.board.get_height() as usize * self.board.get_width() as usize,
        )
        .columns(self.board.get_width() as usize)
        .width(self.board.get_width() as f32 * 16.0);
        for y in 0..self.board.get_height() {
            for x in 0..self.board.get_width() {
                board_view = board_view.push(self.cell(x, y));
            }
        }
        board_view
    }
    fn cell_view(&self, cell: &Cell) -> Element<'_, Action> {
        #[inline]
        fn cell_container<'a>(element: impl Into<Element<'a, Action>>) -> Element<'a, Action> {
            GuiWidget::center(element).width(16).height(16).into()
        }
        cell_container(if cell.is_open() {
            if cell.is_mine() {
                self.exploded_mine()
            } else {
                let mut stack = GuiWidget::Stack::with_capacity(2);
                let open_image = self.opened_cell();
                stack = stack.push(open_image);
                if let Some(adjacent_mines) = cell.adjacent_mines()
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
                        // SAFETY: The internal enum AdjacentMines (which is what
                        // cell.adjacent_mines converts from) CANNOT
                        // represent values outside 0-8, and we just checked
                        // that adjacent_mines is not 0, so all other values are
                        // unreachable.
                        _ => unsafe { std::hint::unreachable_unchecked() },
                    };
                    let text = GuiWidget::text!("{adjacent_mines}")
                        .font(Font::MONOSPACE)
                        .size(14)
                        .color(color);
                    stack = stack.push(cell_container(text));
                }
                cell_container(stack)
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
        })
    }
    fn cell(&self, x: u8, y: u8) -> Element<'_, Action> {
        let cell = self.board.get_cell(x, y).expect("Cell should exist");
        let cell_view = Game::cell_view(self, cell);
        let is_playing = matches!(self.board.get_state(), BoardState::InProgress);
        if is_playing {
            GuiWidget::mouse_area(cell_view)
                .on_press(Action::OpenCell(x, y))
                .on_right_press(Action::ToggleFlag(x, y))
                .on_middle_press(Action::ChordCell(x, y))
                .into()
        } else {
            cell_view
        }
    }
    fn top_menu(&self) -> impl Into<Element<'_, Action>> {
        let remaining_mines = GuiWidget::text!("{}", self.board.get_remaining_mines());
        let reset_button = GuiWidget::button(":)").on_press(Action::ResetGame);
        let time_elapsed = (self.current_time - self.start_time).as_secs();
        let timer = if time_elapsed < 60 {
            GuiWidget::text!("{time_elapsed}").font(Font::MONOSPACE)
        } else if time_elapsed < (99 * 60) + 59 {
            GuiWidget::text!(
                "{minutes}:{seconds:02}",
                minutes = time_elapsed.div_euclid(60),
                seconds = time_elapsed.rem_euclid(60)
            )
            .font(Font::MONOSPACE)
        } else {
            GuiWidget::text("99:59").font(Font::MONOSPACE)
        };
        let content = GuiWidget::row![
            GuiWidget::container(remaining_mines).width(iced::Fill),
            GuiWidget::center_x(reset_button),
            GuiWidget::right(timer)
        ];
        content.width((self.board.get_width() as usize * 16) as f32)
    }
    fn end_of_screen(&self) -> Option<Element<'_, Action>> {
        match self.board.get_state() {
            BoardState::InProgress => None,
            BoardState::Won => {
                let win_text = GuiWidget::text("You found all the mines. You win!");
                let return_button =
                    GuiWidget::button("Return to main menu").on_press(Action::ReturnToMainMenu);
                let content = GuiWidget::column![win_text, return_button].into();
                Some(content)
            }
            BoardState::Lost => {
                let lose_text = GuiWidget::text("You hit a mine! You lose!");
                let return_button =
                    GuiWidget::button("Return to main menu").on_press(Action::ReturnToMainMenu);
                let content = GuiWidget::column![lose_text, return_button].into();
                Some(content)
            }
        }
    }
}
