use iced::*;

#[derive(Default)]
pub struct MinesweeperState {}
type State = MinesweeperState;
type Message = ();

pub fn update(state: &mut State, message: Message) -> impl Into<Task<Message>> {}

pub fn view(state: &State) -> impl Into<Element<'_, Message>> {
    iced::widget::center(widget::text!("minesweeper"))
}
