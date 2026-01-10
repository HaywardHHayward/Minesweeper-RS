use std::sync::Arc;

use GuiWidget::span;
use iced::{Element, Task, widget as GuiWidget};

use super::{AppMessage, MainMenu, Message as SuperMessage};
use crate::{ArcLock, Config, Screen, gui::config};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
}

#[derive(Debug)]
pub struct About {
    config: ArcLock<Config>,
}

impl About {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self { config }
    }
}

impl Screen for About {
    fn update(&mut self, message: SuperMessage) -> Option<Task<SuperMessage>> {
        let SuperMessage::About(message) = message else {
            return None;
        };
        let config = self.config.clone();
        match message {
            Message::Back => Some(
                Task::perform(async { MainMenu::build(config) }, move |item| {
                    Arc::new(Box::new(item) as Box<dyn Screen>)
                })
                .map(AppMessage::ChangeScreen)
                .map(SuperMessage::App),
            ),
        }
    }
    fn view(&self) -> Element<'_, SuperMessage> {
        let menu_theme = &self
            .config
            .read()
            .map(|config| config.menu_theme.clone())
            .unwrap_or(config::MenuTheme::Light);

        let intro_message = menu_theme.text(
            "This application was made using the Rust programming language by Hayden Reckward, using the following libraries:",
        );
        let default_font = menu_theme.default_font();
        let default_size = menu_theme.default_text_size();
        let bold_font = match menu_theme {
            crate::MenuTheme::Light | crate::MenuTheme::Dark => iced::Font {
                weight: iced::font::Weight::Bold,
                ..iced::Font::default()
            },
            crate::MenuTheme::NineX => iced::Font::with_name("Microsoft Sans Serif"),
        };
        let about_message_fn = |[crate_name, crate_author]: [&'static str; 2]| {
            GuiWidget::rich_text![
                span("The "),
                span(crate_name).font(bold_font),
                span(format!(" crate, by {}", crate_author))
            ]
            .font(default_font)
            .size(default_size)
            .on_link_click(iced::never)
        };

        let about_messages = [
            ["rand", "the developers of the Rand project"],
            ["iced", "Héctor Ramón and other Iced contributors"],
            [
                "serde",
                "Erick Tryzelaar, David Tolnay, and all other contributors to Serde",
            ],
            ["serde_yml", "Sebastien Rousseau"],
            ["directories", "Simon Ochsenreither"],
            [
                "zip",
                "Mathijs van de Nes, Marli Frost, Ryan Levick, and Chris Hennick",
            ],
            ["walkdir", "Andrew Gallan"],
            ["tinyvec", "Lokathor"],
            ["thiserror", "David Tolnay"],
            ["ciborium", "Nathaniel McCallum"],
            ["whoami", "the WhoAmI contributors"],
            ["chrono", "Kang Seonghoon and the Chrono contributors"],
            ["clap", "Kevin K. and the Clap contributors"],
            ["rfd", "PolyMeilex"],
            ["anyhow", "David Tolnay"],
        ];

        let library_text = GuiWidget::column(
            about_messages
                .into_iter()
                .map(about_message_fn)
                .map(Element::from),
        );

        let about_text = GuiWidget::column![intro_message, library_text]
            .align_x(iced::Center)
            .spacing(30);

        let return_button = menu_theme
            .button(
                menu_theme.text("Return to main menu"),
                crate::MenuButtonStyle::Secondary,
            )
            .on_press(SuperMessage::About(Message::Back));

        let content = GuiWidget::column![about_text, return_button]
            .align_x(iced::Center)
            .spacing(20);

        GuiWidget::center(content).into()
    }
}
