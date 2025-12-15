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

#[derive(Clone, Debug)]
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

impl LeaderboardEntry {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn time(&self) -> String {
        if self.time.num_seconds() < 60 {
            format!(
                "{}.{:03}",
                self.time.num_seconds(),
                self.time.subsec_millis()
            )
        } else {
            format!(
                "{}:{:02}.{:03}",
                self.time.num_minutes(),
                self.time.num_seconds() % 60,
                self.time.subsec_millis()
            )
        }
    }
    fn completion_date(&self) -> String {
        let local_date = self.completion_date.with_timezone(&chrono::Local);
        local_date.format("%v, %I:%M%p").to_string()
    }
    fn board_size(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }
    fn mines(&self) -> String {
        format!("{}", self.mines)
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Back,
    ChangeTab(Tab),
    ChangeNewEntryName(String),
    SubmitNewEntry,
    CancelNewEntry,
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
        let entries = Self::load_entries().unwrap_or_else(|err| {
            eprintln!("Failed to load leaderboard: {err}");
            BTreeSet::new()
        });
        let new_entry = LeaderboardEntry {
            name: whoami::realname(),
            time,
            completion_date,
            width,
            height,
            mines,
        };
        Self {
            config,
            entries,
            new_entry: Some(new_entry),
            current_tab: Tab::All,
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
            Message::ChangeTab(tab) => {
                self.current_tab = tab;
                None
            }
            Message::ChangeNewEntryName(new_name) => {
                if let Some(ref mut entry) = self.new_entry {
                    entry.name = new_name;
                }
                None
            }
            Message::SubmitNewEntry => {
                if let Some(entry) = self.new_entry.take() {
                    self.entries.insert(entry);
                    let saving_result = self.save_entries();
                    if let Err(err) = saving_result {
                        eprintln!("Failed to save leaderboard: {err}");
                    };
                }
                None
            }
            Message::CancelNewEntry => {
                self.new_entry = None;
                None
            }
        }
    }
    fn view(&self) -> Element<'_, SuperMessage> {
        let config = self.config.read().unwrap();
        let menu_theme = &config.menu_theme;

        let entries = self.entries.iter().rev();
        let table_columns = [
            GuiWidget::table::column(menu_theme.text("Name"), |entry: &LeaderboardEntry| {
                menu_theme.text(entry.name())
            }),
            GuiWidget::table::column(menu_theme.text("Time"), |entry: &LeaderboardEntry| {
                menu_theme.text(entry.time())
            }),
            GuiWidget::table::column(
                menu_theme.text("Completion Date"),
                |entry: &LeaderboardEntry| menu_theme.text(entry.completion_date()),
            ),
            GuiWidget::table::column(menu_theme.text("Board Size"), |entry: &LeaderboardEntry| {
                menu_theme.text(entry.board_size())
            }),
            GuiWidget::table::column(menu_theme.text("Mines"), |entry: &LeaderboardEntry| {
                menu_theme.text(entry.mines())
            }),
        ];
        let entry_elements = entries.filter(|entry| match self.current_tab {
            Tab::All => true,
            Tab::Beginner => entry.width == 9 && entry.height == 9 && entry.mines == 10,
            Tab::Intermediate => entry.width == 16 && entry.height == 16 && entry.mines == 40,
            Tab::Expert => entry.width == 30 && entry.height == 16 && entry.mines == 99,
        });
        let entry_table = GuiWidget::table(table_columns, entry_elements);
        let entries_filter = GuiWidget::row![
            menu_theme
                .button(menu_theme.text("All"), crate::MenuButtonStyle::Primary)
                .on_press(SuperMessage::Leaderboard(Message::ChangeTab(Tab::All))),
            menu_theme
                .button(menu_theme.text("Beginner"), crate::MenuButtonStyle::Primary)
                .on_press(SuperMessage::Leaderboard(Message::ChangeTab(Tab::Beginner))),
            menu_theme
                .button(
                    menu_theme.text("Intermediate"),
                    crate::MenuButtonStyle::Primary
                )
                .on_press(SuperMessage::Leaderboard(Message::ChangeTab(
                    Tab::Intermediate
                ))),
            menu_theme
                .button(menu_theme.text("Expert"), crate::MenuButtonStyle::Primary)
                .on_press(SuperMessage::Leaderboard(Message::ChangeTab(Tab::Expert)))
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center);
        let entries_content = GuiWidget::container(
            GuiWidget::column![
                GuiWidget::center_x(entries_filter),
                GuiWidget::rule::horizontal(1),
                GuiWidget::scrollable(entry_table)
            ]
            .spacing(5),
        )
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

        let popup_window = self.new_entry.as_ref().map(|entry| {
            let popup_text = menu_theme
                .text("Enter your name for the leaderboard:")
                .size(16);

            let name_input_text = menu_theme.text("Your name: ");
            let name_text_input = GuiWidget::text_input("", entry.name().as_str())
                .on_input(|new_name| {
                    SuperMessage::Leaderboard(Message::ChangeNewEntryName(new_name))
                })
                .font(menu_theme.default_font());

            let name_input = GuiWidget::row![name_input_text, name_text_input]
                .spacing(10)
                .align_y(iced::Alignment::Center);

            let main_content = GuiWidget::column![popup_text, name_input]
                .spacing(20)
                .align_x(iced::Alignment::Center);

            let submit_button = menu_theme
                .button(menu_theme.text("Submit"), crate::MenuButtonStyle::Primary)
                .on_press(SuperMessage::Leaderboard(Message::SubmitNewEntry));
            let cancel_button = menu_theme
                .button(
                    menu_theme.text("Discard new entry"),
                    crate::MenuButtonStyle::Secondary,
                )
                .on_press(SuperMessage::Leaderboard(Message::CancelNewEntry));

            let buttons = GuiWidget::row![submit_button, cancel_button]
                .spacing(10)
                .align_y(iced::Alignment::Center);

            GuiWidget::opaque(
                GuiWidget::container(
                    GuiWidget::column![main_content, buttons]
                        .spacing(20)
                        .align_x(iced::Alignment::Center),
                )
                .padding(20)
                .style(GuiWidget::container::bordered_box),
            )
        });

        if let Some(popup) = popup_window {
            GuiWidget::stack![
                GuiWidget::center(content).padding(10),
                GuiWidget::center(popup),
            ]
            .into()
        } else {
            GuiWidget::center(content).padding(10).into()
        }
    }
}
