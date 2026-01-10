use clap::Parser;
mod cli;
mod gui;

pub use gui::{
    Application, ArcLock, Screen, assets,
    config::{Config, GameTheme, MenuButtonStyle, MenuTheme},
    scale_factor, subscription, theme, update, view,
};

pub fn main() -> iced::Result {
    let cli = cli::Cli::parse();
    let boot_fn = move || match cli.get_preset_difficulty() {
        None => Application::create(),
        Some(preset) => match preset {
            cli::PresetDifficulty::Beginner => Application::create_beginner(),
            cli::PresetDifficulty::Intermediate => Application::create_intermediate(),
            cli::PresetDifficulty::Expert => Application::create_expert(),
            cli::PresetDifficulty::Custom => {
                let Some((width, height, mines)) = cli.get_custom_settings() else {
                    panic!("Custom settings must be provided for custom difficulty");
                };
                Application::create_custom(width, height, mines).unwrap_or_else(|err| {
                    eprintln!("Failed to create custom game: {err}");
                    Application::create()
                })
            }
        },
    };
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
        .scale_factor(scale_factor)
        .theme(theme)
        .run()
}
