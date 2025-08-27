﻿use std::{
    num::{NonZeroU8, NonZeroU16},
    sync::Arc,
};

use iced::{Element, Task, widget as GuiWidget};

use super::{AppMessage, Game, GameSelection, Message as SuperMessage};
use crate::{ArcLock, Board, BoardError, Config, Screen};
#[derive(Debug)]
pub struct CustomSetup {
    config: ArcLock<Config>,
    row_string: String,
    column_string: String,
    mines_string: String,
    error_message: Option<Box<str>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    RowChanged(String),
    ColumnChanged(String),
    MinesChanged(String),
    Submit,
}

impl CustomSetup {
    pub fn build(config: ArcLock<Config>) -> Self {
        Self {
            config,
            row_string: String::new(),
            column_string: String::new(),
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
            Message::RowChanged(new_value) => {
                self.row_string = validate_numerical_input::<u8>(&new_value, 2);
                None
            }
            Message::ColumnChanged(new_value) => {
                self.column_string = validate_numerical_input::<u8>(&new_value, 2);
                None
            }
            Message::MinesChanged(new_value) => {
                self.mines_string = validate_numerical_input::<u16>(&new_value, 4);
                None
            }
            Message::Submit => {
                let (row_parsed, column_parsed, mine_parsed) = match (
                    self.row_string.parse::<u8>(),
                    self.column_string.parse::<u8>(),
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
                let (rows, columns, mines) = match (
                    NonZeroU8::new(row_parsed),
                    NonZeroU8::new(column_parsed),
                    NonZeroU16::new(mine_parsed),
                ) {
                    (Some(r), Some(c), Some(m)) => (r, c, m),
                    _ => {
                        self.error_message = Some("All fields must be non-zero.".into());
                        return None;
                    }
                };
                let board = match Board::create_custom(rows, columns, mines) {
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

        let submit_button = GuiWidget::button("Submit")
            .on_press(SuperMessage::CustomSetup(Message::Submit))
            .style(GuiWidget::button::primary);
        let back_button = GuiWidget::button("Back")
            .on_press(SuperMessage::CustomSetup(Message::Back))
            .style(GuiWidget::button::secondary);

        let buttons = GuiWidget::row![submit_button, back_button]
            .spacing(10)
            .align_y(iced::Center);

        let mut content = GuiWidget::column![input_content, buttons]
            .spacing(20)
            .align_x(iced::Center);

        if let Some(error) = &self.error_message {
            let error_text = GuiWidget::text(error.as_ref()).color(iced::color!(0xFF0000));
            content = content.push(error_text);
        }

        GuiWidget::center(content).into()
    }
}
