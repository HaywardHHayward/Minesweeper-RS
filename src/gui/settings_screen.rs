use iced::{Element, Task, widget as GuiWidget};

use crate::gui::{Message as AppMessage, ScreenTrait, ScreenType};
#[derive(Debug)]
pub struct SettingsScreen;

impl Default for SettingsScreen {
    fn default() -> Self {
        SettingsScreen
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    ReturnToMainMenu,
}

impl ScreenTrait for SettingsScreen {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<AppMessage> {
        match message {
            Action::ReturnToMainMenu => Task::done(AppMessage::ChangeScreen(ScreenType::MainMenu)),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let buttons = GuiWidget::button("Return to Main Menu").on_press(Action::ReturnToMainMenu);
        let content = GuiWidget::column![buttons]
            .spacing(20)
            .align_x(iced::Alignment::Center);
        GuiWidget::container(content).center(iced::Fill).into()
    }
}
