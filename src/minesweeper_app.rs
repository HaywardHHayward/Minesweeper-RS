use iced::{widget::*, *};

use crate::Board;

#[derive(Default)]
pub struct MinesweeperState {
    board: Option<Board>,
}
type State = MinesweeperState;
type Message = ();

pub fn update(state: &mut State, message: Message) -> impl Into<Task<Message>> {
    if state.board.is_none() {
        state.board = Board::build(10, 10, 10).ok()
    }
}

pub fn view(state: &State) -> impl Into<Element<Message>> {
    if state.board.is_some() {
        center(
            widget::column![board_ui(state).into(), Button::new("Hello").on_press(())]
                .align_x(Alignment::Center),
        )
    } else {
        center(widget::column![
            widget::text!("minesweeper"),
            Button::new("Hello").on_press(())
        ])
    }
}

fn board_ui(state: &State) -> impl Into<Element<Message>> {
    let board = state.board.as_ref().unwrap();
    let mut grid = Row::with_capacity(board.width() as usize).spacing(2);
    for x in 0..board.width() {
        let mut column = Column::with_capacity(board.height() as usize);
        for y in 0..board.height() {
            column = column.push(text!("{} {}", x, y))
        }
        grid = grid.push(column);
    }
    grid
}
