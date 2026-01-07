use std::sync::Arc;

use GuiWidget::span;
use iced::{Element, Task, widget as GuiWidget};

use super::{AppMessage, MainMenu, Message as SuperMessage};
use crate::{ArcLock, Config, Screen};

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
        let menu_theme = &self.config.read().unwrap().menu_theme;

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
        let about_message_fn = |crate_name, crate_author| {
            GuiWidget::rich_text![
                span("The "),
                span(crate_name).font(bold_font),
                span(format!(" crate, by {}", crate_author))
            ]
            .font(default_font)
            .size(default_size)
            .on_link_click(iced::never)
        };
        let rand = about_message_fn("rand", "the developers of the Rand project");
        let iced = about_message_fn("iced", "Héctor Ramón and other Iced contributors");
        let serde = about_message_fn(
            "serde",
            "Erick Tryzelaar, David Tolnay, and all other contributors to Serde",
        );
        let serde_yml = about_message_fn("serde_yml", "Sebastien Rousseau");
        let directories = about_message_fn("directories", "Simon Ochsenreither");
        let zip = about_message_fn(
            "zip",
            "Mathijs van de Nes, Marli Frost, Ryan Levick, and Chris Hennick",
        );
        let walkdir = about_message_fn("walkdir", "Andrew Gallan");
        let tinyvec = about_message_fn("tinyvec", "Lokathor");
        let thiserror = about_message_fn("thiserror", "David Tolnay");
        let ciborium = about_message_fn("ciborium", "Nathaniel McCallum");
        let whoami = about_message_fn("whoami", "the WhoAmI contributors");
        let chrono = about_message_fn("chrono", "Kang Seonghoon and the Chrono contributors");
        let clap = about_message_fn("clap", "Kevin K. and the Clap contributors");
        let rfd = about_message_fn("rfd", "PolyMeilex");
        let anyhow = about_message_fn("anyhow", "David Tolnay");

        let library_text = GuiWidget::column![
            rand,
            iced,
            serde,
            serde_yml,
            directories,
            zip,
            walkdir,
            tinyvec,
            thiserror,
            ciborium,
            whoami,
            chrono,
            clap,
            rfd,
            anyhow
        ];

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
