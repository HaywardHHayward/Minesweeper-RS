use std::sync::Arc;

use iced::{Element, Task, widget as GuiWidget};

use super::{AppMessage, GameSelection, Message as SuperMessage};
use crate::{ArcLock, Config, Screen};
#[derive(Debug)]
pub struct CustomSetup {
    config: ArcLock<Config>,
    row_string: String,
    column_string: String,
    mines_string: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    RowChanged(String),
    ColumnChanged(String),
    MinesChanged(String),
}

impl CustomSetup {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self {
            config,
            row_string: String::new(),
            column_string: String::new(),
            mines_string: String::new(),
        }
    }
}

fn remove_non_digits(input: String) -> String {
    input.chars().filter(|char| char.is_ascii_digit()).collect()
}

impl Screen for CustomSetup {
    fn update(&mut self, message: SuperMessage) -> Option<Task<SuperMessage>> {
        let SuperMessage::CustomSetup(message) = message else {
            return None;
        };
        let config = self.config.clone();
        match message {
            Message::Back => Some(Task::perform(
                async { GameSelection::build(config) },
                move |item| SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item)))),
            )),
            Message::RowChanged(new_value) => {
                if new_value.len() > 2 {
                    return None;
                }
                self.row_string = remove_non_digits(new_value);
                None
            }
            Message::ColumnChanged(new_value) => {
                if new_value.len() > 2 {
                    return None;
                }
                self.column_string = remove_non_digits(new_value);
                None
            }
            Message::MinesChanged(new_value) => {
                if new_value.len() > 4 {
                    return None;
                }
                self.mines_string = remove_non_digits(new_value);
                None
            }
        }
    }
    fn view(&self) -> Element<'_, SuperMessage> {
        let row_text = GuiWidget::text("Rows:");
        let row_input = GuiWidget::text_input("", &self.row_string)
            .on_input(|new_value| SuperMessage::CustomSetup(Message::RowChanged(new_value)));

        let column_text = GuiWidget::text("Columns:");
        let column_input = GuiWidget::text_input("", &self.column_string)
            .on_input(|new_value| SuperMessage::CustomSetup(Message::ColumnChanged(new_value)));

        let mines_text = GuiWidget::text("Mines:");
        let mines_input = GuiWidget::text_input("", &self.mines_string)
            .on_input(|new_value| SuperMessage::CustomSetup(Message::MinesChanged(new_value)));

        let inputs = GuiWidget::column![row_input, column_input, mines_input]
            .spacing(10)
            .align_x(iced::Right)
            .width(60);
        let texts = GuiWidget::column![row_text, column_text, mines_text]
            .spacing(20)
            .align_x(iced::Left);

        let input_content = GuiWidget::row![texts, inputs]
            .spacing(10)
            .align_y(iced::Center);

        let back_button = GuiWidget::button("Back")
            .on_press(SuperMessage::CustomSetup(Message::Back))
            .style(GuiWidget::button::secondary);

        let content = GuiWidget::column![input_content, back_button]
            .spacing(20)
            .align_x(iced::Center);

        GuiWidget::center(content).into()
    }
}
