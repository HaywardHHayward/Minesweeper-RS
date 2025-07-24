use minesweeper_rs::*;

pub fn main() -> iced::Result {
    iced::application(Application::default, update, view)
        .settings(iced::Settings::default())
        .subscription(subscription)
        .theme(theme)
        .run()
}
