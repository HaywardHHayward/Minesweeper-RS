use super::AppMessage;

macro_rules! screen_macro {
    ($([$snake_case:ident, $pascal_case:ident]),*) => {
        $(pub mod $snake_case;)*

        #[derive(Debug)]
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
    [about, About]
);
