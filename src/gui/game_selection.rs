use iced::{Element, Task, widget as GuiWidget};

use crate::gui::{Message as AppMessage, ScreenMessage, ScreenTrait};

#[derive(Debug, Default)]
pub struct GameSelection {
    state: GameSelectionImpl,
}

#[derive(Debug, Default)]
enum GameSelectionImpl {
    #[default]
    OptionSelection,
    CustomSelection(CustomSelection),
}

#[derive(Debug, Default)]
struct CustomSelection {
    row: String,
    column: String,
    mines: String,
}

#[derive(Debug, Clone)]
enum TextChangedEnum {
    Row,
    Column,
    Mines,
}

#[derive(Debug, Clone)]
pub enum Action {
    StartGame(Options),
    GoToCustom,
    GoToOptionSelection,
    CheckCustom,
    ReturnToMainMenu,
    TextChanged(TextChangedEnum, String),
}

impl ScreenTrait for GameSelection {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<AppMessage> {
        match message {
            Action::StartGame(options) => {
                let board = match options {
                    Options::Beginner => crate::core::board::Board::create_beginner(),
                    Options::Intermediate => crate::core::board::Board::create_intermediate(),
                    Options::Expert => crate::core::board::Board::create_expert(),
                    Options::Custom => {
                        // Short-circuit, go to custom board screen for setup
                        return Task::done(AppMessage::ScreenAction(ScreenMessage::GameSelection(
                            Action::GoToCustom,
                        )));
                    }
                };
                let game = crate::gui::game::Game::new(board);
                Task::done(AppMessage::InitializeScreen {
                    screen_type: crate::gui::ScreenType::Game,
                    initializer_fn: Box::new(|| crate::gui::Screen::Game(game)),
                    change_screen: true,
                })
            }
            Action::GoToCustom => {
                // Transition to custom selection screen
                self.state = GameSelectionImpl::CustomSelection(CustomSelection::default());
                Task::none()
            }
            Action::GoToOptionSelection => {
                // Return to option selection
                self.state = GameSelectionImpl::OptionSelection;
                Task::none()
            }
            Action::CheckCustom => {
                // Validate custom options and start the game if valid
                Task::none()
            }
            Action::ReturnToMainMenu => {
                Task::done(AppMessage::ChangeScreen(crate::gui::ScreenType::MainMenu))
            }
            Action::TextChanged(selection, string) => {
                let GameSelectionImpl::CustomSelection(CustomSelection {
                    ref mut row,
                    ref mut column,
                    ref mut mines,
                }) = self.state
                else {
                    return Task::none();
                };
                let numeric_string = string
                    .matches(|ref char| char::is_ascii_digit(char))
                    .collect::<String>();
                match selection {
                    TextChangedEnum::Row => {
                        if numeric_string.len() <= 2 {
                            *row = numeric_string;
                        }
                    }
                    TextChangedEnum::Column => {
                        if numeric_string.len() <= 2 {
                            *column = numeric_string;
                        }
                    }
                    TextChangedEnum::Mines => {
                        if numeric_string.len() <= 4 {
                            *mines = numeric_string;
                        }
                    }
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self.state {
            GameSelectionImpl::OptionSelection => {
                let beginner = GuiWidget::button("Beginner")
                    .on_press(Action::StartGame(Options::Beginner))
                    .width(iced::Fill);
                let intermediate = GuiWidget::button("Intermediate")
                    .on_press(Action::StartGame(Options::Intermediate))
                    .width(iced::Fill);
                let expert = GuiWidget::button("Expert")
                    .on_press(Action::StartGame(Options::Expert))
                    .width(iced::Fill);
                let custom = GuiWidget::button("Custom")
                    .on_press(Action::StartGame(Options::Custom))
                    .width(iced::Fill);
                let options = GuiWidget::column![beginner, intermediate, expert, custom].width(110);
                let return_to_menu =
                    GuiWidget::button("Return to main menu").on_press(Action::ReturnToMainMenu);
                let content = GuiWidget::column![options, return_to_menu].spacing(20);
                GuiWidget::container(content).center(iced::Fill).into()
            }
            GameSelectionImpl::CustomSelection(ref custom) => {
                const EDITOR_WIDTH: f32 = 60.0;
                let row_text = GuiWidget::text!("Rows: ");
                let row_editor = GuiWidget::text_input("", &custom.row)
                    .on_input(|input| Action::TextChanged(TextChangedEnum::Row, input))
                    .width(iced::Fill);
                let column_text = GuiWidget::text("Columns: ");
                let column_editor = GuiWidget::text_input("", &custom.column)
                    .on_input(|input| Action::TextChanged(TextChangedEnum::Column, input))
                    .width(iced::Fill);
                let mines_text = GuiWidget::text("Mines: ");
                let mines_editor = GuiWidget::text_input("", &custom.mines)
                    .on_input(|input| Action::TextChanged(TextChangedEnum::Mines, input))
                    .width(iced::Fill);
                let texts = GuiWidget::column![row_text, column_text, mines_text]
                    .spacing(10)
                    .align_x(iced::Left);
                let editors = GuiWidget::column![row_editor, column_editor, mines_editor]
                    .width(EDITOR_WIDTH)
                    .align_x(iced::Right);
                let custom_editors = GuiWidget::row![texts, editors]
                    .height(iced::Shrink)
                    .align_y(iced::Center);
                let create_button = GuiWidget::button("Create board").on_press_maybe(
                    if !custom.row.is_empty()
                        && !custom.column.is_empty()
                        && !custom.mines.is_empty()
                    {
                        Some(Action::CheckCustom)
                    } else {
                        None
                    },
                );
                let custom_content = GuiWidget::column![custom_editors, create_button]
                    .align_x(iced::Center)
                    .spacing(10);
                let return_button =
                    GuiWidget::button("Return to options").on_press(Action::GoToOptionSelection);
                let content = GuiWidget::column![custom_content, return_button]
                    .align_x(iced::Center)
                    .spacing(20);
                GuiWidget::container(content).center(iced::Fill).into()
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Options {
    Beginner,
    Intermediate,
    Expert,
    Custom,
}
