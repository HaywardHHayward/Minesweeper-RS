use iced::{Element, Task, futures::FutureExt, widget as GuiWidget};

use crate::gui::{Message as AppMessage, ScreenTrait, ScreenType};

#[derive(Debug)]
pub struct MainMenu;

#[derive(Debug, Clone)]
pub enum Action {
    StartGame,
    Settings,
    Exit,
}

impl ScreenTrait for MainMenu {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<AppMessage> {
        match message {
            Action::StartGame => {
                // TODO: Start the game, have screen to chose difficulty
                Task::done(AppMessage::InitializeScreen {
                    screen_type: ScreenType::Game,
                    initializer_fn: {
                        || {
                            Box::pin(
                                super::game::initialize_game(super::game::Options::Beginner)
                                    .map(super::Screen::Game),
                            )
                        }
                    },
                    change_screen: true,
                })
            }
            Action::Settings => {
                // TODO: Provide function to initialize settings screen, then change screen to
                // it
                Task::done(AppMessage::ChangeScreen(ScreenType::Settings))
            }
            Action::Exit => iced::exit(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = GuiWidget::text!("Minesweeper").size(50);
        let buttons = GuiWidget::column![
            GuiWidget::button("Start Game").on_press(Action::StartGame),
            GuiWidget::button("Settings").on_press(Action::Settings),
            GuiWidget::button("Exit").on_press(Action::Exit),
        ]
        .spacing(5)
        .align_x(iced::Alignment::Center);
        let content = GuiWidget::column![title, buttons]
            .spacing(20)
            .align_x(iced::Alignment::Center);
        GuiWidget::container(content).center(iced::Fill).into()
    }
}
