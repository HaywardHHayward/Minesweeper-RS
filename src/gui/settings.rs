use std::io::Read;

use directories::ProjectDirs;
use iced::{Element, Task, widget as GuiWidget};

use crate::gui::{Message as AppMessage, ScreenTrait, ScreenType};
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Settings;

impl Default for Settings {
    fn default() -> Self {
        // TODO: Create default settings, will be used when the settings file does not
        // exist
        Settings
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    ReturnToMainMenu,
}

impl ScreenTrait for Settings {
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

pub(crate) async fn initialize_settings() -> Settings {
    let project_directories = ProjectDirs::from("", "HaywardHHayward", "Minesweeper")
        .expect("Failed to get project directories");
    let config_dir = project_directories.config_dir();
    if !config_dir.exists() {
        std::fs::create_dir_all(config_dir).expect("Failed to create config directory");
    }
    let config_fp = config_dir.join("settings.yaml");
    if !config_fp.exists() {
        // TODO: Create a default settings file, and return default settings
        let settings = Settings::default();
        let serialized_settings =
            serde_yml::to_string(&settings).expect("Failed to serialize default settings");
        std::fs::write(&config_fp, serialized_settings).expect("Failed to create settings file");
        settings
    } else {
        // TODO: Parse the settings file and load settings
        let mut config_file =
            std::fs::File::open(&config_fp).expect("Failed to open settings file");
        let mut content = String::new();
        config_file
            .read_to_string(&mut content)
            .expect("Failed to read settings file");
        serde_yml::from_str::<Settings>(content.as_str()).expect("Failed to parse settings file")
    }
}
