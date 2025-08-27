use minesweeper_rs::*;

pub fn main() -> iced::Result {
    iced::application(Application::create, update, view)
        .title("Minesweeper")
        .settings(iced::Settings::default())
        .window(iced::window::Settings {
            icon: iced::window::icon::from_file_data(assets::ICON, None).ok(),
            exit_on_close_request: false,
            ..Default::default()
        })
        .subscription(subscription)
        .theme(theme)
        .scale_factor(scale_factor)
        .run()
}
