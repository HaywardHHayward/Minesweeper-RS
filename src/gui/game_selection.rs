use std::num::{IntErrorKind, NonZeroU8, NonZeroU16};

use iced::{Element, Task, widget as GuiWidget};

use crate::{
    core::board::{Board, BoardError},
    gui::{Message as AppMessage, ScreenMessage, ScreenTrait},
};

#[derive(Debug, Default)]
pub(crate) struct GameSelection {
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
    height: String,
    width: String,
    mines: String,
    error: Option<Vec<GameSelectionError>>,
}

#[derive(Debug)]
enum GameSelectionError {
    BoardCreate(BoardError),
    IsZero(TextOptions),
}

#[derive(Debug, Clone)]
pub(crate) enum TextOptions {
    Height,
    Width,
    Mines,
}

#[derive(Debug, Clone)]
pub(crate) enum Action {
    StartGame(Options),
    GoToCustom,
    GoToOptionSelection,
    CheckCustom,
    ReturnToMainMenu,
    TextChanged(TextOptions, String),
}

impl ScreenTrait for GameSelection {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<AppMessage> {
        match message {
            Self::Message::StartGame(options) => {
                let board = match options {
                    Options::Beginner => Board::create_beginner(),
                    Options::Intermediate => Board::create_intermediate(),
                    Options::Expert => Board::create_expert(),
                    Options::Custom => {
                        // Short-circuit, go to custom board screen for setup
                        return Task::done(AppMessage::ScreenAction(ScreenMessage::GameSelection(
                            Self::Message::GoToCustom,
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
            Self::Message::GoToCustom => {
                // Transition to custom selection screen
                self.state = GameSelectionImpl::CustomSelection(CustomSelection::default());
                Task::none()
            }
            Self::Message::GoToOptionSelection => {
                // Return to option selection
                self.state = GameSelectionImpl::OptionSelection;
                Task::none()
            }
            Self::Message::CheckCustom => {
                let GameSelectionImpl::CustomSelection(CustomSelection {
                    ref height,
                    ref width,
                    ref mines,
                    ref mut error,
                }) = self.state
                else {
                    return Task::none();
                };
                let height_result = height.parse::<NonZeroU8>().map_err(|error| {
                    match error.kind() {
                        IntErrorKind::Zero => GameSelectionError::IsZero(TextOptions::Height),
                        _ => unreachable!("Only cause of error should be that the height is zero. Received cause of error: {:?}", error.kind()),
                    }
                });
                let width_result = width.parse::<NonZeroU8>().map_err(|error| {
                    match error.kind() {
                        IntErrorKind::Zero => GameSelectionError::IsZero(TextOptions::Width),
                        _ => unreachable!("Only cause of error should be that the width is zero. Received cause of error: {:?}", error.kind())
                    }
                });
                let mines_result = mines.parse::<NonZeroU16>().map_err(|error| {
                    match error.kind() {
                        IntErrorKind::Zero => GameSelectionError::IsZero(TextOptions::Mines),
                        _ => unreachable!("Only cause of error should be that the mines is zero. Received cause of error: {:?}", error.kind())
                    }
                });
                if let Ok(height_num) = height_result
                    && let Ok(width_num) = width_result
                    && let Ok(mines_num) = mines_result
                {
                    let board_result = Board::create_custom(width_num, height_num, mines_num);
                    match board_result {
                        Ok(board) => {
                            let game = crate::gui::game::Game::new(board);
                            Task::done(AppMessage::InitializeScreen {
                                screen_type: crate::gui::ScreenType::Game,
                                initializer_fn: Box::new(|| crate::gui::Screen::Game(game)),
                                change_screen: true,
                            })
                        }
                        Err(board_error) => {
                            *error = Some(vec![GameSelectionError::BoardCreate(board_error)]);
                            Task::none()
                        }
                    }
                } else {
                    let mut error_vec = Vec::with_capacity(3);
                    if let Err(height_error) = height_result {
                        error_vec.push(height_error);
                    }
                    if let Err(width_error) = width_result {
                        error_vec.push(width_error);
                    }
                    if let Err(mines_error) = mines_result {
                        error_vec.push(mines_error);
                    }
                    *error = Some(error_vec);
                    Task::none()
                }
            }
            Self::Message::ReturnToMainMenu => {
                Task::done(AppMessage::ChangeScreen(crate::gui::ScreenType::MainMenu))
            }
            Self::Message::TextChanged(selection, string) => {
                let GameSelectionImpl::CustomSelection(CustomSelection {
                    ref mut height,
                    ref mut width,
                    ref mut mines,
                    ..
                }) = self.state
                else {
                    return Task::none();
                };
                let numeric_string = string
                    .matches(|ref char| char::is_ascii_digit(char))
                    .collect::<String>();
                match selection {
                    TextOptions::Height => {
                        if numeric_string.len() <= 2 {
                            *height = numeric_string;
                        }
                    }
                    TextOptions::Width => {
                        if numeric_string.len() <= 2 {
                            *width = numeric_string;
                        }
                    }
                    TextOptions::Mines => {
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
                let beginner = GuiWidget::button(GuiWidget::center_x("Beginning"))
                    .on_press(Self::Message::StartGame(Options::Beginner));
                let intermediate =
                    GuiWidget::button(GuiWidget::center_x("Intermediate").width(iced::Shrink))
                        .on_press(Self::Message::StartGame(Options::Intermediate));
                let expert = GuiWidget::button(GuiWidget::center_x("Expert"))
                    .on_press(Self::Message::StartGame(Options::Expert));
                let custom = GuiWidget::button(GuiWidget::center_x("Custom"))
                    .on_press(Self::Message::StartGame(Options::Custom));

                let options =
                    GuiWidget::column![beginner, intermediate, expert, custom].width(iced::Shrink);

                let return_to_menu = GuiWidget::button("Return to main menu")
                    .on_press(Self::Message::ReturnToMainMenu);

                let content = GuiWidget::column![options, return_to_menu]
                    .align_x(iced::Center)
                    .spacing(20);
                GuiWidget::center(content).into()
            }
            GameSelectionImpl::CustomSelection(CustomSelection {
                ref height,
                ref width,
                ref mines,
                ref error,
            }) => {
                const EDITOR_WIDTH: f32 = 60.0;
                let width_text = GuiWidget::text("Width: ");
                let width_editor = GuiWidget::text_input("", width)
                    .on_input(|input| Self::Message::TextChanged(TextOptions::Width, input))
                    .width(iced::Fill);

                let height_text = GuiWidget::text("Height: ");
                let height_editor = GuiWidget::text_input("", height)
                    .on_input(|input| Self::Message::TextChanged(TextOptions::Height, input))
                    .width(iced::Fill);

                let mines_text = GuiWidget::text("Mines: ");
                let mines_editor = GuiWidget::text_input("", mines)
                    .on_input(|input| Self::Message::TextChanged(TextOptions::Mines, input))
                    .width(iced::Fill);

                let texts = GuiWidget::column![width_text, height_text, mines_text]
                    .spacing(10)
                    .align_x(iced::Left);
                let editors = GuiWidget::column![width_editor, height_editor, mines_editor]
                    .width(EDITOR_WIDTH)
                    .align_x(iced::Right);

                let custom_editors = GuiWidget::row![texts, editors]
                    .height(iced::Shrink)
                    .align_y(iced::Center);

                let create_button = GuiWidget::button("Create board").on_press_maybe(
                    if !height.is_empty() && !width.is_empty() && !mines.is_empty() {
                        Some(Self::Message::CheckCustom)
                    } else {
                        None
                    },
                );

                let custom_content = GuiWidget::column![custom_editors, create_button]
                    .align_x(iced::Center)
                    .spacing(10);

                let possible_errors = error.as_deref();
                let error_message = possible_errors.map(|errors| {
                    let mut error_string = String::new();
                    for error in errors {
                        match error {
                            GameSelectionError::IsZero(field) => {
                                let field_text = match field {
                                    TextOptions::Height => "height",
                                    TextOptions::Width => "width",
                                    TextOptions::Mines => "mines"
                                };
                                error_string += format!("Cannot have zero {field_text}\n").as_str()
                            }
                            GameSelectionError::BoardCreate(BoardError::InvalidBoardSize) => {
                                error_string += "Cannot have a 1x1 board\n"
                            }
                            GameSelectionError::BoardCreate(BoardError::TooManyMines(maximum_mines)) => {
                                error_string += format!("Too many mines. The maximum amount of mines at the given board size is {maximum_mines}").as_str()
                            }
                        }
                    }
                    error_string
                });

                let return_button = GuiWidget::button("Return to options")
                    .on_press(Self::Message::GoToOptionSelection);

                let content = if let Some(error_message) = error_message {
                    GuiWidget::column![
                        custom_content,
                        GuiWidget::text(error_message),
                        return_button
                    ]
                } else {
                    GuiWidget::column![custom_content, return_button]
                };
                GuiWidget::center(content.align_x(iced::Center).spacing(20)).into()
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum Options {
    Beginner,
    Intermediate,
    Expert,
    Custom,
}
