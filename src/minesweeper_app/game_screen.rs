use iced::*;

use super::*;
use crate::Board;
pub struct GameScreen {
    board: Board,
}

impl GameScreen {
    pub fn new(width: u8, height: u8, mine_count: u16) -> GameScreen {
        Self {
            board: Board::build(width, height, mine_count).unwrap(),
        }
    }
    fn display_board(&self) -> impl Into<Element<Message>> {
        let mut row = Row::with_capacity(self.board.width() as usize).spacing(2);
        for x in 0..self.board.width() {
            let mut col = Column::with_capacity(self.board.height() as usize);
            for y in 0..self.board.height() {
                let ref_tile = self.board.get(x, y).unwrap();
                let message = if !ref_tile.is_open() {
                    if ref_tile.is_flagged() {
                        " P".into()
                    } else {
                        "[]".into()
                    }
                } else if ref_tile.is_mined() {
                    " *".into()
                } else {
                    let value = ref_tile.surrounding_mines().unwrap();
                    if value == 0 {
                        "  ".into()
                    } else {
                        format!(" {}", value)
                    }
                };
                let element = MouseArea::new(text!("{}", message))
                    .on_press(Message::OpenTile(x, y))
                    .on_middle_press(Message::SafeOpenTile(x, y))
                    .on_right_press(Message::FlagTile(x, y));
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
            Message::OpenTile(x, y) => {
                self.board.open_tile(x, y);
            }
            Message::FlagTile(x, y) => {
                self.board.toggle_flag(x, y);
            }
            Message::SafeOpenTile(x, y) => {
                self.board.open_safe(x, y);
            }
            _ => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        center(self.display_board()).into()
    }
}
