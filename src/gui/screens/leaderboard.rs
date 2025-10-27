use std::sync::Arc;

use chrono::{DateTime, TimeDelta};
use iced::{widget as GuiWidget, Element, Task};

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
    fn entry_element(&self, entry: &LeaderboardEntry) -> Element<'_, SuperMessage> {
        let font = self.config.read().unwrap().menu_theme.default_font();
        let name = GuiWidget::text(entry.name.clone()).font(font);
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
        let time = GuiWidget::text(time_string).font(font);
        let local_date = entry.completion_date.with_timezone(&chrono::Local);
        let date_string = local_date.format("%v, %I:%M%p").to_string();
        let date = GuiWidget::text(date_string).font(font);
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

        let mut entries = self.entries.clone();
        // if let Some(new_entry) = &self.new_entry {
        //     entries.push(new_entry.clone());
        // }
        entries.sort_by_key(|entry| entry.time);
        let entry_elements = entries
            .into_iter()
            .map(|entry| self.entry_element(&entry))
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
