use clap::Parser;
use minesweeper::*;

// TODO: Add support for command line arguments

pub fn main() -> iced::Result {
    let cli = cli::Cli::parse();
    let boot_fn = cli.get_preset_difficulty().map_or(
        Application::default as fn() -> Application,
        |difficulty| match difficulty {
            cli::PresetDifficulty::Beginner => Application::create_beginner,
            cli::PresetDifficulty::Intermediate => Application::create_intermediate,
            cli::PresetDifficulty::Expert => Application::create_expert,
        },
    );
    iced::application(boot_fn, update, view)
        .title("Minesweeper")
        .settings(iced::Settings {
            id: Some("com.github.haywardhhayward.Minesweeper".to_string()),
            ..Default::default()
        })
        .window(iced::window::Settings {
            size: iced::Size::new(600f32, 400f32),
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
