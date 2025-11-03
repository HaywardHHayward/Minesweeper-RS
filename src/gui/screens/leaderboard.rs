use std::{collections::BTreeSet, sync::Arc};

use chrono::{DateTime, TimeDelta};
use iced::{Element, Task, widget as GuiWidget};

use super::{AppMessage, MainMenu, Message as SuperMessage};
use crate::{Application, ArcLock, Config, Screen};
#[derive(Debug)]
pub struct Leaderboard {
    config: ArcLock<Config>,
    entries: BTreeSet<LeaderboardEntry>,
    new_entry: Option<LeaderboardEntry>,
    current_tab: Tab,
}

#[derive(Debug)]
pub enum Tab {
    All,
    Beginner,
    Intermediate,
    Expert,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
struct LeaderboardEntry {
    #[serde(rename = "n")]
    name: String,
    #[serde(rename = "t")]
    time: TimeDelta,
    #[serde(with = "chrono::serde::ts_seconds", rename = "d")]
    completion_date: DateTime<chrono::Utc>,
    #[serde(rename = "w")]
    width: u8,
    #[serde(rename = "h")]
    height: u8,
    #[serde(rename = "m")]
    mines: u16,
}

impl PartialOrd for LeaderboardEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LeaderboardEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sorts by time (descending), then by board size + mines (ascending), then by
        // completion date (descending), then by name (ascending)
        other
            .time
            .cmp(&self.time)
            .then(
                (self.width as u16 * self.height as u16 + self.mines)
                    .cmp(&(other.width as u16 * other.height as u16 + other.mines)),
            )
            .then(other.completion_date.cmp(&self.completion_date))
            .then(self.name.cmp(&other.name))
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Back,
}

impl Leaderboard {
    fn load_entries() -> Result<BTreeSet<LeaderboardEntry>, Box<dyn std::error::Error>> {
        let path = Application::app_dirs()
            .data_dir()
            .join("leaderboard")
            .to_path_buf();
        if !path.exists() {
            return Ok(BTreeSet::new());
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
    pub fn delete_entries() -> Result<(), Box<dyn std::error::Error>> {
        let path = Application::app_dirs()
            .data_dir()
            .join("leaderboard")
            .to_path_buf();
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }
    pub fn from_menu(config: ArcLock<Config>) -> Self {
        let entries = Self::load_entries().unwrap_or_else(|err| {
            eprintln!("Failed to load leaderboard: {err}");
            BTreeSet::new()
        });
        Self {
            config,
            entries,
            new_entry: None,
            current_tab: Tab::All,
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
            BTreeSet::new()
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
        entries.insert(new_entry.clone());
        Self {
            config,
            entries,
            new_entry: Some(new_entry),
            current_tab: Tab::All,
        }
    }
    fn entry_element(&self, entry: &LeaderboardEntry) -> Element<'_, SuperMessage> {
        let config = &self.config.read().unwrap().menu_theme;
        let name = config.text(entry.name.clone());
        let time_string = if entry.time.num_seconds() < 60 {
            format!(
                "{}.{:03}",
                entry.time.num_seconds(),
                entry.time.subsec_millis()
            )
        } else {
            format!(
                "{}:{:02}.{:03}",
                entry.time.num_minutes(),
                entry.time.num_seconds() % 60,
                entry.time.subsec_millis()
            )
        };
        let time = config.text(time_string);
        let local_date = entry.completion_date.with_timezone(&chrono::Local);
        let date_string = local_date.format("%v, %I:%M%p").to_string();
        let date = config.text(date_string);
        GuiWidget::row![name, time, date]
            .spacing(20)
            .align_y(iced::Alignment::Center)
            .into()
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
    fn view(&self) -> Element<'_, SuperMessage> {
        let config = self.config.read().unwrap();
        let menu_theme = &config.menu_theme;

        let entries = self.entries.iter().rev();
        let entry_elements = entries
            .filter_map(|entry| match &self.current_tab {
                Tab::All => Some(self.entry_element(entry)),
                Tab::Beginner => {
                    if entry.width == 9 && entry.height == 9 && entry.mines == 10 {
                        Some(self.entry_element(entry))
                    } else {
                        None
                    }
                }
                Tab::Intermediate => {
                    if entry.width == 16 && entry.height == 16 && entry.mines == 40 {
                        Some(self.entry_element(entry))
                    } else {
                        None
                    }
                }
                Tab::Expert => {
                    if entry.width == 30 && entry.height == 16 && entry.mines == 99 {
                        Some(self.entry_element(entry))
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<_>>();
        let entries_column = GuiWidget::column(entry_elements)
            .spacing(10)
            .height(iced::Fill)
            .width(iced::Fill);
        let entries_content = GuiWidget::container(GuiWidget::scrollable(entries_column))
            .padding(10)
            .height(iced::Fill)
            .width(iced::Fill)
            .style(GuiWidget::container::bordered_box);

        let back_button = menu_theme
            .button(menu_theme.text("Back"), crate::MenuButtonStyle::Secondary)
            .on_press(SuperMessage::Leaderboard(Message::Back));

        let content = GuiWidget::column![entries_content, back_button]
            .spacing(20)
            .align_x(iced::Alignment::Center);

        GuiWidget::center(content).padding(10).into()
    }
}
