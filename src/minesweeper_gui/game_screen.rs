use iced::{widget::*, *};

use crate::{minesweeper_core::Board, minesweeper_gui::Screen};
pub struct GameScreen {
    board: Board,
    start_time: Option<time::Instant>,
    current_time: Option<time::Instant>,
}

impl GameScreen {
    pub fn new(width: u8, height: u8, mines: u16) -> Self {
        Self {
            board: Board::build(width, height, mines).unwrap(),
            start_time: None,
            current_time: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenTile(u8, u8),
    ToggleTileFlag(u8, u8),
    SafeOpenTile(u8, u8),
    CurrentTime(time::Instant),
}

impl Screen for GameScreen {
    type ScreenMessage = Message;
    type ScreenAction = Action;

    fn update(&mut self, message: Self::ScreenMessage) -> Self::ScreenAction {
        match message {
            Message::OpenTile(x, y) => {
                self.board.open_tile(x, y);
            }
            Message::ToggleTileFlag(x, y) => {
                self.board.toggle_flag(x, y);
            }
            Message::SafeOpenTile(x, y) => {
                self.board.open_safe(x, y);
            }
            Message::CurrentTime(time) => {
                if self.start_time.is_none() {
                    self.start_time = Some(time);
                }
                self.current_time = Some(time);
            }
        }
        Action::None
    }

    fn view(&self) -> Element<Self::ScreenMessage> {
        let timer = if let [Some(start), Some(now)] = [self.start_time, self.current_time] {
            text!(
                "Time elapsed: {}",
                now.saturating_duration_since(start).as_secs()
            )
        } else {
            text!("Time elapsed: 0")
        };
        center(widget::column![self.display_board(), timer]).into()
    }
}

impl GameScreen {
    fn display_tile(&self, x: u8, y: u8) -> Element<Message> {
        let tile = self.board.get(x, y).unwrap();
        let display = if tile.is_open() {
            if tile.is_mined() {
                "*".into()
            } else if tile.surrounding_mines().unwrap() == 0 {
                " ".into()
            } else {
                format!("{}", tile.surrounding_mines().unwrap())
            }
        } else if tile.is_flagged() {
            "P".into()
        } else {
            "O".into()
        };
        MouseArea::new(Text::new(display))
            .on_press(Message::OpenTile(x, y))
            .on_middle_press(Message::SafeOpenTile(x, y))
            .on_right_press(Message::ToggleTileFlag(x, y))
            .into()
    }
    fn display_board(&self) -> Element<Message> {
        let mut full_board = Column::with_capacity(self.board.height() as usize);
        for y in 0..self.board.height() {
            let mut row = Row::with_capacity(self.board.width() as usize);
            for x in 0..self.board.width() {
                let tile = self.display_tile(x, y);
                row = row.push(tile);
            }
            row = row.spacing(5);
            full_board = full_board.push(row);
        }
        full_board.into()
    }
}
