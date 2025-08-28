use std::{fmt::Display, fs::File, path::Path};

use iced::{
    widget as GuiWidget,
    widget::button::{Status as ButtonStatus, Style as ButtonStyle},
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub game_theme: GameTheme,
    pub menu_theme: MenuTheme,
    pub scale_factor: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub enum GameTheme {
    SimpleLight,
    SimpleDark,
    #[cfg(feature = "non-free")]
    Classic,
}

impl GameTheme {
    pub const ALL: &'static [GameTheme] = &[
        GameTheme::SimpleLight,
        GameTheme::SimpleDark,
        #[cfg(feature = "non-free")]
        GameTheme::Classic,
    ];
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub enum MenuTheme {
    Light,
    Dark,
    NineX,
}

pub enum MenuButtonStyle {
    Primary,
    Secondary,
    Danger,
}

impl MenuTheme {
    pub const ALL: &'static [MenuTheme] = &[MenuTheme::Light, MenuTheme::Dark, MenuTheme::NineX];

    pub fn theme(&self) -> iced::Theme {
        match self {
            MenuTheme::Light => iced::Theme::Light,
            MenuTheme::Dark => iced::Theme::Dark,
            MenuTheme::NineX => iced::Theme::custom(
                "9x",
                iced::theme::Palette {
                    background: iced::color!(0xC0C0C0),
                    text: iced::color!(0x222222),
                    ..iced::theme::Palette::DARK
                },
            ),
        }
    }

    pub fn button_style(
        &'_ self,
        button_style: MenuButtonStyle,
    ) -> GuiWidget::button::StyleFn<'_, iced::Theme> {
        match self {
            MenuTheme::Light | MenuTheme::Dark => match button_style {
                MenuButtonStyle::Primary => Box::new(GuiWidget::button::primary),
                MenuButtonStyle::Secondary => Box::new(GuiWidget::button::secondary),
                MenuButtonStyle::Danger => Box::new(GuiWidget::button::danger),
            },
            MenuTheme::NineX => {
                let base_button_style = ButtonStyle {
                    background: Some(iced::Background::Color(iced::color!(0xc0c0c0))),
                    text_color: iced::color!(0x222222),
                    border: iced::Border::default()
                        .color(iced::color!(0xDFDFDF))
                        .width(2),
                    shadow: iced::Shadow {
                        color: iced::color!(0x808080),
                        offset: iced::Vector::new(2.0, 2.0),
                        blur_radius: 0.0,
                    },
                    snap: true,
                };
                Box::new(move |_theme, status| match status {
                    ButtonStatus::Active | ButtonStatus::Hovered => base_button_style,
                    ButtonStatus::Pressed => {
                        let mut pressed_style = base_button_style;
                        pressed_style.shadow = iced::Shadow {
                            color: iced::color!(0xDFDFDF),
                            offset: iced::Vector::new(-2.0, -2.0),
                            blur_radius: 0.0,
                        };
                        pressed_style.border.color = iced::color!(0x808080);
                        pressed_style
                    }
                    ButtonStatus::Disabled => {
                        let mut disabled_style = base_button_style;
                        disabled_style.background =
                            Some(iced::Background::Color(iced::color!(0xE0E0E0)));
                        disabled_style.text_color = iced::color!(0xA0A0A0);
                        disabled_style.border = iced::Border::default()
                            .color(iced::color!(0xA0A0A0))
                            .width(2);
                        disabled_style.shadow = iced::Shadow {
                            color: iced::color!(0xC0C0C0),
                            offset: iced::Vector::new(0.0, 0.0),
                            blur_radius: 0.0,
                        };
                        disabled_style
                    }
                })
            }
        }
    }
}

impl Display for GameTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GameTheme::SimpleLight => "Simple (Light)",
            GameTheme::SimpleDark => "Simple (Dark)",
            #[cfg(feature = "non-free")]
            GameTheme::Classic => "Classic",
        })
    }
}

impl Display for MenuTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            MenuTheme::Light => "Light",
            MenuTheme::Dark => "Dark",
            MenuTheme::NineX => "9x",
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            game_theme: GameTheme::SimpleLight,
            menu_theme: MenuTheme::Light,
            scale_factor: 1.0,
        }
    }
}

impl Config {
    pub fn save(&self, save_location: &Path) {
        let save_file = File::create(save_location).expect("Failed to create config file");
        serde_yml::to_writer(save_file, &self).expect("Failed to serialize config");
    }

    pub fn load(load_location: &Path) -> Result<Self, serde_yml::Error> {
        let config_file = File::open(load_location).expect("Failed to open config file");
        let config = serde_yml::from_reader(config_file)?;
        Ok(config)
    }
}
