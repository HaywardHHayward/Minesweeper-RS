use iced::*;

use super::*;
use crate::Board;

#[derive(Debug, Clone, Copy)]
pub enum GameMessage {
    OpenTile(u8, u8),
    FlagTile(u8, u8),
    SafeOpenTile(u8, u8),
    GameFinished { won: bool },
}
pub struct GameScreen {
    board: Board,
    seconds_elapsed: u64,
    start_time: Option<time::Instant>,
}

impl GameScreen {
    pub fn new(width: u8, height: u8, mine_count: u16) -> GameScreen {
        Self {
            board: Board::build(width, height, mine_count).unwrap(),
            seconds_elapsed: 0,
            start_time: None,
        }
    }
    fn display_tile(&self, x: u8, y: u8) -> impl Into<Element<Message>> {
        let tile = self.board.get(x, y).unwrap();
        let graphic = if !tile.is_open() {
            if tile.is_flagged() {
                " P".into()
            } else {
                "[]".into()
            }
        } else if tile.is_mined() {
            " *".into()
        } else {
            let value = tile.surrounding_mines().unwrap();
            if value == 0 {
                "  ".into()
            } else {
                format!(" {}", value)
            }
        };
        text!("{}", graphic)
    }
    fn display_board(&self) -> impl Into<Element<Message>> {
        let mut row = Row::with_capacity(self.board.width() as usize).spacing(2);
        for x in 0..self.board.width() {
            let mut col = Column::with_capacity(self.board.height() as usize);
            for y in 0..self.board.height() {
                let element = MouseArea::new(self.display_tile(x, y))
                    .on_press(Message::Game(GameMessage::OpenTile(x, y)))
                    .on_middle_press(Message::Game(GameMessage::SafeOpenTile(x, y)))
                    .on_right_press(Message::Game(GameMessage::FlagTile(x, y)));
                col = col.push(element);
            }
            row = row.push(col);
        }
        row
    }
}

impl Screen for GameScreen {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Game(game_message) => match game_message {
                GameMessage::OpenTile(x, y) => {
                    self.board.open_tile(x, y);
                }
                GameMessage::FlagTile(x, y) => {
                    self.board.toggle_flag(x, y);
                }
                GameMessage::SafeOpenTile(x, y) => {
                    self.board.open_safe(x, y);
                }
                _ => {}
            },
            Message::ChangeScreen(ScreenChoices::Game) => {
                self.start_time = Some(time::Instant::now());
            }
            Message::QueryingTime(instant) => {
                self.seconds_elapsed = instant
                    .saturating_duration_since(self.start_time.unwrap())
                    .as_secs()
            }
            _ => {}
        }
        if !self.board.is_playing() {
            return Task::done(Message::DeactivateTimer)
                .chain(Task::done(Message::Game(GameMessage::GameFinished {
                    won: !self.board.hit_mine(),
                })))
                .chain(Task::done(Message::ChangeScreen(ScreenChoices::StartMenu)));
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let content = widget::column![
            text!("{}", self.seconds_elapsed),
            self.display_board().into()
        ];
        center(content).into()
    }
}
