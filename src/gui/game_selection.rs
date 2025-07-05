use std::num::{NonZeroU8, NonZeroU16};

use iced::Task;

use crate::gui::{Message, ScreenTrait};

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
    row: Option<NonZeroU8>,
    column: Option<NonZeroU8>,
    mines: Option<NonZeroU16>,
}

#[derive(Debug, Clone)]
pub enum Action {
    StartGame(Options),
    GoToCustom,
    GoToOptionSelection,
    CheckCustom,
    ReturnToMainMenu,
}

impl ScreenTrait for GameSelection {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<Message> {
        match message {
            Action::StartGame(options) => {
                let board = match options {
                    Options::Beginner => crate::core::board::Board::create_beginner(),
                    Options::Intermediate => crate::core::board::Board::create_intermediate(),
                    Options::Expert => crate::core::board::Board::create_expert(),
                    Options::Custom => {
                        // Short-circuit, go to custom board screen for setup
                        return Task::none();
                    }
                };
                let game = crate::gui::game::Game::new(board);
                Task::done(Message::InitializeScreen {
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
                Task::done(Message::ChangeScreen(crate::gui::ScreenType::MainMenu))
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
