use iced::{widget::*, *};

use crate::minesweeper_gui::Screen;

pub struct StartScreen {}

impl StartScreen {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PressedButton(Action),
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    None,
    MakeBoard { width: u8, height: u8, mines: u16 },
}

impl Screen for StartScreen {
    type ScreenMessage = Message;
    type ScreenAction = Action;

    fn update(&mut self, message: Self::ScreenMessage) -> Self::ScreenAction {
        match message {
            Message::PressedButton(action) => action,
        }
    }

    fn view(&self) -> Element<Self::ScreenMessage> {
        let beginner = Button::new(Text::new("Beginner")).on_press(Message::PressedButton(
            Action::MakeBoard {
                width: 9,
                height: 9,
                mines: 10,
            },
        ));
        let intermediate = Button::new(Text::new("Intermediate")).on_press(Message::PressedButton(
            Action::MakeBoard {
                width: 16,
                height: 16,
                mines: 40,
            },
        ));
        let expert =
            Button::new(Text::new("Expert")).on_press(Message::PressedButton(Action::MakeBoard {
                width: 30,
                height: 16,
                mines: 99,
            }));
        center(widget::column![beginner, intermediate, expert].align_x(Center)).into()
    }
}
