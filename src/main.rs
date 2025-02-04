use minesweeper_rs::gui::*;

pub fn main() -> iced::Result {
    iced::application("Minesweeper", update, view).run()
}
