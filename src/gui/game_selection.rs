use iced::{widget as GuiWidget, Element, Task};

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
    height: String,
    width: String,
    mines: String,
}

#[derive(Debug, Clone)]
enum TextChangedEnum {
    Height,
    Width,
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
            Self::Message::StartGame(options) => {
                let board = match options {
                    Options::Beginner => crate::core::board::Board::create_beginner(),
                    Options::Intermediate => crate::core::board::Board::create_intermediate(),
                    Options::Expert => crate::core::board::Board::create_expert(),
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
                // Validate custom options and start the game if valid
                Task::none()
            }
            Self::Message::ReturnToMainMenu => {
                Task::done(AppMessage::ChangeScreen(crate::gui::ScreenType::MainMenu))
            }
            Self::Message::TextChanged(selection, string) => {
                let GameSelectionImpl::CustomSelection(CustomSelection {
                    ref mut height,
                    ref mut width,
                    ref mut mines,
                }) = self.state
                else {
                    return Task::none();
                };
                let numeric_string = string
                    .matches(|ref char| char::is_ascii_digit(char))
                    .collect::<String>();
                match selection {
                    TextChangedEnum::Height => {
                        if numeric_string.len() <= 2 {
                            *height = numeric_string;
                        }
                    }
                    TextChangedEnum::Width => {
                        if numeric_string.len() <= 2 {
                            *width = numeric_string;
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
                let beginner = GuiWidget::button(centered_text("Beginning", iced::Fill))
                    .on_press(Self::Message::StartGame(Options::Beginner));
                let intermediate = GuiWidget::button(centered_text("Intermediate", iced::Shrink))
                    .on_press(Self::Message::StartGame(Options::Intermediate));
                let expert = GuiWidget::button(centered_text("Expert", iced::Fill))
                    .on_press(Self::Message::StartGame(Options::Expert));
                let custom = GuiWidget::button(centered_text("Custom", iced::Fill))
                    .on_press(Self::Message::StartGame(Options::Custom));

                let options =
                    GuiWidget::column![beginner, intermediate, expert, custom].width(iced::Shrink);

                let return_to_menu = GuiWidget::button("Return to main menu")
                    .on_press(Self::Message::ReturnToMainMenu);

                let content = GuiWidget::column![options, return_to_menu]
                    .align_x(iced::Center)
                    .spacing(20);
                GuiWidget::container(content).center(iced::Fill).into()
            }
            GameSelectionImpl::CustomSelection(ref custom) => {
                const EDITOR_WIDTH: f32 = 60.0;
                let width_text = GuiWidget::text("Width: ");
                let width_editor = GuiWidget::text_input("", &custom.width)
                    .on_input(|input| Self::Message::TextChanged(TextChangedEnum::Width, input))
                    .width(iced::Fill);

                let height_text = GuiWidget::text("Height: ");
                let height_editor = GuiWidget::text_input("", &custom.height)
                    .on_input(|input| Self::Message::TextChanged(TextChangedEnum::Height, input))
                    .width(iced::Fill);

                let mines_text = GuiWidget::text("Mines: ");
                let mines_editor = GuiWidget::text_input("", &custom.mines)
                    .on_input(|input| Self::Message::TextChanged(TextChangedEnum::Mines, input))
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
                    if !custom.height.is_empty()
                        && !custom.width.is_empty()
                        && !custom.mines.is_empty()
                    {
                        Some(Self::Message::CheckCustom)
                    } else {
                        None
                    },
                );

                let custom_content = GuiWidget::column![custom_editors, create_button]
                    .align_x(iced::Center)
                    .spacing(10);

                let return_button = GuiWidget::button("Return to options")
                    .on_press(Self::Message::GoToOptionSelection);

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
#[inline]
fn centered_text<'a>(
    text: impl Into<GuiWidget::Text<'a>>,
    fill_strategy: iced::Length,
) -> impl Into<Element<'a, Action>> {
    GuiWidget::container(text.into())
        .align_x(iced::Center)
        .width(fill_strategy)
}
