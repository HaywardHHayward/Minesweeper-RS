use iced::{Element, Task, widget as GuiWidget};

use crate::gui::{Message as AppMessage, ScreenTrait, ScreenType};

#[derive(Debug, Default)]
pub(crate) struct MainMenu;

#[derive(Debug, Clone)]
pub(crate) enum Action {
    StartGame,
    Settings,
    About,
    Exit,
}

impl ScreenTrait for MainMenu {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<AppMessage> {
        match message {
            Self::Message::StartGame => Task::done(AppMessage::InitializeScreen {
                screen_type: ScreenType::GameSelection,
                initializer_fn: Box::new(|_| {
                    crate::gui::Screen::GameSelection(
                        crate::gui::game_selection::GameSelection::default(),
                    )
                }),
            }),
            Self::Message::Settings => {
                Task::done(AppMessage::ChangeScreen(ScreenType::SettingsScreen))
            }
            Self::Message::About => Task::done(AppMessage::ChangeScreen(ScreenType::About)),
            Self::Message::Exit => iced::exit(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = GuiWidget::text("Minesweeper").size(50);
        let buttons = GuiWidget::column![
            GuiWidget::button("Start Game").on_press(Self::Message::StartGame),
            GuiWidget::button("Settings").on_press(Self::Message::Settings),
            GuiWidget::button("About").on_press(Self::Message::About),
            GuiWidget::button("Exit").on_press(Self::Message::Exit),
        ]
        .spacing(5)
        .align_x(iced::Alignment::Center);
        let content = GuiWidget::column![title, buttons]
            .spacing(20)
            .align_x(iced::Alignment::Center);
        GuiWidget::center(content).into()
    }
}
