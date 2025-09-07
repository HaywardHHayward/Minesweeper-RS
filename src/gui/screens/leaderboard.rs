use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use iced::Task;

use super::{AppMessage, MainMenu, Message as SuperMessage};
use crate::{Application, ArcLock, Config, Screen};
#[derive(Debug)]
pub struct Leaderboard {
    config: ArcLock<Config>,
    entries: Vec<LeaderboardEntry>,
    new_entry: Option<LeaderboardEntry>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct LeaderboardEntry {
    name: String,
    time: Duration,
    completion_time: SystemTime,
    width: u8,
    height: u8,
    mines: u16,
}

#[derive(Clone, Debug)]
pub enum Message {
    Back,
}

impl Leaderboard {
    fn load_entries() -> Result<Vec<LeaderboardEntry>, Box<dyn std::error::Error>> {
        let path = Application::app_dirs().data_dir().join("leaderboard");
        if !path.exists() {
            return Ok(Vec::new());
        }
        let file = std::fs::File::open(path)?;
        let data = ciborium::from_reader(file)?;
        Ok(data)
    }
    fn save_entries(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Application::app_dirs().data_dir().join("leaderboard");
        let file = std::fs::File::create(path)?;
        ciborium::into_writer(&self.entries, file)?;
        Ok(())
    }
    pub fn from_menu(config: ArcLock<Config>) -> Self {
        let entries = Self::load_entries().unwrap_or_else(|err| {
            eprintln!("Failed to load leaderboard: {err}");
            Vec::new()
        });
        Self {
            config,
            entries,
            new_entry: None,
        }
    }
    pub fn from_new_time(
        config: ArcLock<Config>,
        time: Duration,
        completion_time: SystemTime,
        (width, height, mines): (u8, u8, u16),
    ) -> Self {
        let entries = Self::load_entries().unwrap_or_else(|err| {
            eprintln!("Failed to load leaderboard: {err}");
            Vec::new()
        });
        Self {
            config,
            entries,
            new_entry: Some(LeaderboardEntry {
                name: whoami::realname(),
                time,
                completion_time,
                width,
                height,
                mines,
            }),
        }
    }
}

impl Screen for Leaderboard {
    fn update(&mut self, message: SuperMessage) -> Option<Task<SuperMessage>> {
        let SuperMessage::Leaderboard(message) = message else {
            return None;
        };
        let config = self.config.clone();
        match message {
            Message::Back => Some(
                Task::perform(async { MainMenu::build(config) }, move |item| {
                    Arc::new(Box::new(item) as Box<dyn Screen>)
                })
                .map(AppMessage::ChangeScreen)
                .map(SuperMessage::App),
            ),
        }
    }
}
