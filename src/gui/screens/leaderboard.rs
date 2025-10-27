use std::{default, sync::Arc};

use chrono::{DateTime, TimeDelta};
use iced::{Element, Task, widget as GuiWidget};

use super::{AppMessage, MainMenu, Message as SuperMessage};
use crate::{Application, ArcLock, Config, Screen};
#[derive(Debug)]
pub struct Leaderboard {
    config: ArcLock<Config>,
    entries: Vec<LeaderboardEntry>,
    new_entry: Option<LeaderboardEntry>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct LeaderboardEntry {
    name: String,
    time: TimeDelta,
    #[serde(with = "chrono::serde::ts_seconds")]
    completion_date: DateTime<chrono::Utc>,
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
        let path = Application::app_dirs()
            .data_dir()
            .join("leaderboard")
            .to_path_buf();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let file = std::fs::File::open(path)?;
        let data = ciborium::from_reader(file)?;
        Ok(data)
    }
    fn save_entries(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data_dir = Application::app_dirs().data_dir().to_path_buf();
        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir).expect("Unable to create data directory");
        }
        let path = data_dir.join("leaderboard");
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
        time: TimeDelta,
        completion_date: DateTime<chrono::Utc>,
        (width, height, mines): (u8, u8, u16),
    ) -> Self {
        let mut entries = Self::load_entries().unwrap_or_else(|err| {
            eprintln!("Failed to load leaderboard: {err}");
            Vec::new()
        });
        // TODO: This will add new entry into entries for now whilst I design a way to
        // modify entries before having them inserted into the master list
        let new_entry = LeaderboardEntry {
            name: whoami::realname(),
            time,
            completion_date,
            width,
            height,
            mines,
        };
        entries.push(new_entry.clone());
        Self {
            config,
            entries,
            new_entry: Some(new_entry),
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
            Message::Back => {
                let saving_result = self.save_entries();
                if let Err(err) = saving_result {
                    eprintln!("Failed to save leaderboard: {err}");
                };
                Some(
                    Task::perform(async { MainMenu::build(config) }, move |item| {
                        Arc::new(Box::new(item) as Box<dyn Screen>)
                    })
                    .map(AppMessage::ChangeScreen)
                    .map(SuperMessage::App),
                )
            }
        }
    }
}
