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
        let rand = GuiWidget::rich_text![
            span("The "),
            span("rand").font(bold_font),
            span(" crate, by the developers of the Rand project")
        ]
        .font(default_font)
        .size(default_size)
        .on_link_click(iced::never);
        let iced = GuiWidget::rich_text![
            span("The "),
            span("iced").font(bold_font),
            span(" crate, by Héctor Ramón and other Iced contributors")
        ]
        .font(default_font)
        .size(default_size)
        .on_link_click(iced::never);
        let serde = GuiWidget::rich_text![
            span("The "),
            span("serde").font(bold_font),
            span(" crate, by Erick Tryzelaar, David Tolnay, and all other contributors to Serde")
        ]
        .font(default_font)
        .size(default_size)
        .on_link_click(iced::never);
        let serde_yml = GuiWidget::rich_text![
            span("The "),
            span("serde_yml").font(bold_font),
            span(
                " crate (a fork of the `serde_yaml` crate by David Tolnay), by Sebastien Rousseau"
            )
        ]
        .font(default_font)
        .size(default_size)
        .on_link_click(iced::never);
        let directories = GuiWidget::rich_text![
            span("The "),
            span("directories").font(bold_font),
            span(" crate, by Simon Ochsenreither")
        ]
        .font(default_font)
        .size(default_size)
        .on_link_click(iced::never);
        let zip = GuiWidget::rich_text![
            span("The "),
            span("zip").font(bold_font),
            span(" crate, by Mathijs van de Nes, Marli Frost, Ryan Levick, and Chris Hennick")
        ]
        .font(default_font)
        .size(default_size)
        .on_link_click(iced::never);
        let walkdir = GuiWidget::rich_text![
            span("The "),
            span("walkdir").font(bold_font),
            span(" crate, by Andrew Gallan")
        ]
        .font(default_font)
        .size(default_size)
        .on_link_click(iced::never);
        let tinyvec = GuiWidget::rich_text![
            span("The "),
            span("tinyvec").font(bold_font),
            span(" crate, by Lokathor")
        ]
        .font(default_font)
        .size(default_size)
        .on_link_click(iced::never);
        let thiserror = GuiWidget::rich_text![
            span("The "),
            span("thiserror").font(bold_font),
            span(" crate, by David Tolnay")
        ]
        .font(default_font)
        .size(default_size)
        .on_link_click(iced::never);
        let ciborium = GuiWidget::rich_text![
            span("The "),
            span("ciborium").font(bold_font),
            span(" crate, by Nathaniel McCallum")
        ]
        .font(default_font)
        .size(default_size)
        .on_link_click(iced::never);
        let whoami = GuiWidget::rich_text![
            span("The "),
            span("whoami").font(bold_font),
            span(" crate, by the WhoAmI contributors")
        ]
        .font(default_font)
        .size(default_size)
        .on_link_click(iced::never);

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
            whoami
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
