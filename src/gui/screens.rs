use super::AppMessage;

macro_rules! screen_macro {
    ($([$snake_case:ident, $pascal_case:ident]),*) => {
        pub use self::{
            $(
                $snake_case::$pascal_case,
            )*
        };
        $(pub mod $snake_case;)*

        #[derive(Debug, Clone)]
        pub enum Message {
            App(AppMessage),
            $(
                $pascal_case($snake_case::Message),
            )*
        }
    }
}

screen_macro!(
    [main_menu, MainMenu],
    [settings_screen, SettingsScreen],
    [game_selection, GameSelection],
    [game, Game],
    [about, About],
    [custom_setup, CustomSetup]
);
