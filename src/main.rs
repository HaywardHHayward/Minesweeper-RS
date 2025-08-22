use minesweeper_rs::*;

pub fn main() -> iced::Result {
    iced::application(Application::create, update, view)
        .settings(iced::Settings::default())
        .subscription(subscription)
        .theme(theme)
        .scale_factor(scale_factor)
        .run()
}
