use std::{
    num::{NonZeroU8, NonZeroU16},
    sync::Arc,
};

use iced::{Element, Task, widget as GuiWidget};

use super::{AppMessage, Game, GameSelection, Message as SuperMessage};
use crate::{ArcLock, Board, BoardError, Config, Screen};
#[derive(Debug)]
pub struct CustomSetup {
    config: ArcLock<Config>,
    width_string: String,
    height_string: String,
    mines_string: String,
    error_message: Option<Box<str>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    HeightChanged(String),
    WidthChanged(String),
    MinesChanged(String),
    Submit,
}

impl CustomSetup {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self {
            config,
            width_string: String::new(),
            height_string: String::new(),
            mines_string: String::new(),
            error_message: None,
        }
    }
}

#[inline]
fn remove_non_digits(input: &str) -> String {
    input.chars().filter(|char| char.is_ascii_digit()).collect()
}

fn validate_numerical_input<T>(input: &str, max_length: usize) -> String
where
    T: Into<usize> + std::str::FromStr,
{
    if input.len() > max_length {
        return input[0..max_length].to_string();
    }
    let filtered = remove_non_digits(input);
    if filtered.is_empty() {
        return String::new();
    }
    if let Ok(value) = filtered.parse::<T>() {
        if value.into() == 0 {
            return "0".to_string();
        }
        filtered.trim_start_matches('0').to_string()
    } else {
        String::new()
    }
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
            Message::WidthChanged(new_value) => {
                self.width_string = validate_numerical_input::<u8>(&new_value, 2);
                None
            }
            Message::HeightChanged(new_value) => {
                self.height_string = validate_numerical_input::<u8>(&new_value, 2);
                None
            }
            Message::MinesChanged(new_value) => {
                self.mines_string = validate_numerical_input::<u16>(&new_value, 4);
                None
            }
            Message::Submit => {
                let (width_parsed, height_parsed, mine_parsed) = match (
                    self.width_string.parse::<u8>(),
                    self.height_string.parse::<u8>(),
                    self.mines_string.parse::<u16>(),
                ) {
                    (Ok(r), Ok(c), Ok(m)) => (r, c, m),
                    _ => {
                        // The only way the parsing could fail to my knowledge is if the strings are
                        // empty, due to pre-validation of each of the strings.
                        self.error_message = Some("All fields must be filled.".into());
                        return None;
                    }
                };
                let (width, height, mines) = match (
                    NonZeroU8::new(width_parsed),
                    NonZeroU8::new(height_parsed),
                    NonZeroU16::new(mine_parsed),
                ) {
                    (Some(r), Some(c), Some(m)) => (r, c, m),
                    _ => {
                        self.error_message = Some("All fields must be non-zero.".into());
                        return None;
                    }
                };
                let board = match Board::create_custom(width, height, mines) {
                    Ok(board) => board,
                    Err(BoardError::InvalidBoardSize) => {
                        self.error_message =
                            Some("Invalid board size! Rows and columns cannot both be one.".into());
                        return None;
                    }
                    Err(BoardError::TooManyMines { max_mines }) => {
                        self.error_message = Some(
                            format!("Too many mines! Maximum for the given rows and columns is {max_mines}.").into(),
                        );
                        return None;
                    }
                };
                Some(Task::perform(
                    async move { Game::build(config, board) },
                    move |item| {
                        SuperMessage::App(AppMessage::ChangeScreen(Arc::new(Box::new(item))))
                    },
                ))
            }
        }
    }
    fn view(&self) -> Element<'_, SuperMessage> {
        let width_text = GuiWidget::text("Width:");
        let width_input = GuiWidget::text_input("", &self.width_string)
            .on_input(|new_value| SuperMessage::CustomSetup(Message::WidthChanged(new_value)));

        let height_text = GuiWidget::text("Height:");
        let height_input = GuiWidget::text_input("", &self.height_string)
            .on_input(|new_value| SuperMessage::CustomSetup(Message::HeightChanged(new_value)));

        let mines_text = GuiWidget::text("Mines:");
        let mines_input = GuiWidget::text_input("", &self.mines_string)
            .on_input(|new_value| SuperMessage::CustomSetup(Message::MinesChanged(new_value)));

        let inputs = GuiWidget::column![width_input, height_input, mines_input]
            .spacing(10)
            .align_x(iced::Right)
            .width(60);
        let texts = GuiWidget::column![width_text, height_text, mines_text]
            .spacing(20)
            .align_x(iced::Left);

        let input_content = GuiWidget::row![texts, inputs]
            .spacing(10)
            .align_y(iced::Center);

        let submit_button = GuiWidget::button("Submit")
            .on_press(SuperMessage::CustomSetup(Message::Submit))
            .style(|theme, status| {
                self.config
                    .read()
                    .unwrap()
                    .menu_theme
                    .button_style(crate::gui::config::MenuButtonStyle::Primary)(
                    theme, status
                )
            });
        let back_button = GuiWidget::button("Back")
            .on_press(SuperMessage::CustomSetup(Message::Back))
            .style(|theme, status| {
                self.config
                    .read()
                    .unwrap()
                    .menu_theme
                    .button_style(crate::gui::config::MenuButtonStyle::Secondary)(
                    theme, status
                )
            });

        let buttons = GuiWidget::row![submit_button, back_button]
            .spacing(10)
            .align_y(iced::Center);

        let error_message = GuiWidget::text(self.error_message.as_deref().unwrap_or(""));

        let content = GuiWidget::column![
            input_content,
            GuiWidget::vertical_space().height(10),
            error_message,
            buttons
        ]
        .spacing(10)
        .align_x(iced::Center);

        GuiWidget::center(content).into()
    }
}
