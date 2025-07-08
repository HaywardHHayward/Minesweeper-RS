use iced::{Element, Task, widget as GuiWidget};

use crate::gui::{Message as AppMessage, ScreenTrait};

#[derive(Clone, Debug)]
pub enum Action {
    ReturnToMainMenu,
}

#[derive(Debug, Default)]
pub struct About;

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

        let rand = GuiWidget::text("The `rand` crate, by the developers of the Rand project");
        let iced = GuiWidget::text("The `iced` crate, by Héctor Ramón and other Iced contributors");
        let serde = GuiWidget::text(
            "The `serde` crate, by Erick Tryzelaar, David Tolnay, and all other contributors to Serde",
        );
        let serde_yml = GuiWidget::text(
            "The `serde_yml` crate (a fork of the `serde_yaml` crate by David Tolnay), by Sebastien Rousseau",
        );
        let directories = GuiWidget::text("The `directories` crate, by Simon Ochsenreither");

        let library_text =
            GuiWidget::column![rand, iced, serde, serde_yml, directories,].align_x(iced::Center);

        let about_text = GuiWidget::column![intro_message, library_text]
            .align_x(iced::Center)
            .spacing(30);

        let return_button =
            GuiWidget::button("Return to main menu").on_press(Self::Message::ReturnToMainMenu);

        let content = GuiWidget::column![about_text, return_button]
            .align_x(iced::Center)
            .spacing(20);

        GuiWidget::container(content).center(iced::Fill).into()
    }
}
