use minesweeper_rs::*;

pub fn main() -> iced::Result {
    iced::application(Application::default, update, view)
        .settings(iced::Settings::default())
        .subscription(subscription)
        .theme(theme)
        .scale_factor(scale_factor)
        .exit_on_close_request(false)
        .run()
}
