use minesweeper_rs::*;

pub fn main() -> iced::Result {
    iced::application(Application::create, update, view)
        .title("Minesweeper")
        .settings(iced::Settings {
            id: Some("com.github.haywardhhayward.Minesweeper".to_string()),
            ..Default::default()
        })
        .window(iced::window::Settings {
            icon: iced::window::icon::from_file_data(assets::ICON, None)
                .inspect_err(|err| eprintln!("Failed to load icon: {err}"))
                .ok(),
            exit_on_close_request: false,
            ..Default::default()
        })
        .subscription(subscription)
        .theme(theme)
        .scale_factor(scale_factor)
        .run()
}
