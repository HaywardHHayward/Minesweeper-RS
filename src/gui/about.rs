use iced::{Element, Task, widget as GuiWidget, widget::span};

use crate::gui::{Message as AppMessage, ScreenTrait};

#[derive(Clone, Debug)]
pub(crate) enum Action {
    ReturnToMainMenu,
}

#[derive(Debug, Default)]
pub(crate) struct About;

impl ScreenTrait for About {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<AppMessage> {
        match message {
            Self::Message::ReturnToMainMenu => {
                Task::done(AppMessage::ChangeScreen(crate::gui::ScreenType::MainMenu))
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let intro_message = GuiWidget::text(
            "This application was made using the Rust programming language by Hayden Reckward, using the following libraries:",
        );
        let bold_font = iced::Font {
            weight: iced::font::Weight::Bold,
            ..iced::Font::default()
        };
        let rand = GuiWidget::rich_text![
            span("The "),
            span("rand").font(bold_font),
            span(" crate, by the developers of the Rand project")
        ]
        .on_link_click(iced::never);
        let iced = GuiWidget::rich_text![
            span("The "),
            span("iced").font(bold_font),
            span(" crate, by Héctor Ramón and other Iced contributors")
        ]
        .on_link_click(iced::never);
        let serde = GuiWidget::rich_text![
            span("The "),
            span("serde").font(bold_font),
            span(" crate, by Erick Tryzelaar, David Tolnay, and all other contributors to Serde")
        ]
        .on_link_click(iced::never);
        let serde_yml = GuiWidget::rich_text![
            span("The "),
            span("serde_yml").font(bold_font),
            span(
                " crate (a fork of the `serde_yaml` crate by David Tolnay), by Sebastien Rousseau"
            )
        ]
        .on_link_click(iced::never);
        let directories = GuiWidget::rich_text![
            span("The "),
            span("directories").font(bold_font),
            span(" crate, by Simon Ochsenreither")
        ]
        .on_link_click(iced::never);
        let zip = GuiWidget::rich_text![
            span("The "),
            span("zip").font(bold_font),
            span(" crate, by Mathijs van de Nes, Marli Frost, Ryan Levick, and Chris Hennick")
        ]
        .on_link_click(iced::never);
        let walkdir = GuiWidget::rich_text![
            span("The "),
            span("walkdir").font(bold_font),
            span(" crate, by Andrew Gallan")
        ]
        .on_link_click(iced::never);
        let local_vec = GuiWidget::rich_text![
            span("The "),
            span("local_vec").font(bold_font),
            span(" crate, by Jorge Rinaldi")
        ]
        .on_link_click(iced::never);

        let library_text = GuiWidget::column![
            rand,
            iced,
            serde,
            serde_yml,
            directories,
            zip,
            walkdir,
            local_vec
        ];

        let about_text = GuiWidget::column![intro_message, library_text]
            .align_x(iced::Center)
            .spacing(30);

        let return_button =
            GuiWidget::button("Return to main menu").on_press(Self::Message::ReturnToMainMenu);

        let content = GuiWidget::column![about_text, return_button]
            .align_x(iced::Center)
            .spacing(20);

        GuiWidget::center(content).into()
    }
}
