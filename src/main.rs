use minesweeper_rs::gui::*;

pub fn main() -> iced::Result {
    iced::application("Minesweeper", update, view)
        .settings(iced::Settings::default())
        .subscription(subscription)
        .run()
}
