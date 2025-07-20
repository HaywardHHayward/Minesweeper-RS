use std::{
    num::{NonZeroU8, NonZeroU16},
    time::Instant,
};

use iced::{Element, Font, Subscription, Task, widget as GuiWidget, widget::image as GuiImage};

use crate::{
    core::{board::*, cell::*},
    gui::{Message as AppMessage, ScreenTrait, ScreenType},
};

#[derive(Debug)]
pub(crate) struct Game {
    board: Board,
    start_time: Instant,
    current_time: Instant,
}

#[derive(Debug, Clone)]
pub(crate) enum Action {
    OpenCell(u8, u8),
    ToggleFlag(u8, u8),
    ChordCell(u8, u8),
    ResetGame,
    TimeUpdate(Instant),
    ReturnToMainMenu,
}

impl Game {
    pub(crate) fn new(board: Board) -> Self {
        let game_start = Instant::now();
        Self {
            board,
            start_time: game_start,
            current_time: game_start,
        }
    }
}

impl ScreenTrait for Game {
    type Message = Action;

    fn update(&mut self, message: Self::Message) -> Task<AppMessage> {
        match message {
            Self::Message::OpenCell(x, y) => {
                self.board.open_cell(x, y);
                Task::none()
            }
            Self::Message::ToggleFlag(x, y) => {
                self.board.toggle_flag(x, y);
                Task::none()
            }
            Self::Message::ChordCell(x, y) => {
                self.board.chord_cell(x, y);
                Task::none()
            }
            Self::Message::ResetGame => {
                let (rows, columns, mine_count) = (
                    self.board.get_height(),
                    self.board.get_width(),
                    self.board.get_mine_count(),
                );
                let new_board = Board::create_custom(
                    NonZeroU8::new(columns).unwrap(),
                    NonZeroU8::new(rows).unwrap(),
                    NonZeroU16::new(mine_count).unwrap(),
                )
                .unwrap();
                let new_start = Instant::now();
                self.start_time = new_start;
                self.current_time = new_start;
                self.board = new_board;
                Task::none()
            }
            Self::Message::TimeUpdate(time) => {
                self.current_time = time;
                Task::none()
            }
            Self::Message::ReturnToMainMenu => {
                Task::done(AppMessage::ChangeScreen(ScreenType::MainMenu))
            }
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        let mut board_view = GuiWidget::Column::with_capacity(self.board.get_height() as usize);
        for y in 0..self.board.get_height() {
            let mut row = GuiWidget::Row::with_capacity(self.board.get_width() as usize);
            for x in 0..self.board.get_width() {
                row = row.push(self.cell(x, y));
            }
            board_view = board_view.push(row);
        }
        let mut game_content =
            GuiWidget::column![self.top_menu().into(), board_view].align_x(iced::Center);
        let extra_content = match self.board.get_state() {
            BoardState::InProgress => None,
            BoardState::Won => {
                let win_text = GuiWidget::text("You found all the mines. You win!");
                let return_button = GuiWidget::button("Return to main menu")
                    .on_press(Self::Message::ReturnToMainMenu);
                let content = GuiWidget::column![win_text, return_button].align_x(iced::Center);
                Some(content)
            }
            BoardState::Lost => {
                let lose_text = GuiWidget::text("You hit a mine! You lose!");
                let return_button = GuiWidget::button("Return to main menu")
                    .on_press(Self::Message::ReturnToMainMenu);
                let content = GuiWidget::column![lose_text, return_button].align_x(iced::Center);
                Some(content)
            }
        };
        game_content = game_content.push_maybe(extra_content);
        let content = GuiWidget::center(game_content);
        content.into()
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        match self.board.get_state() {
            BoardState::InProgress => {
                iced::time::every(std::time::Duration::from_secs(1)).map(Self::Message::TimeUpdate)
            }
            BoardState::Won | BoardState::Lost => Subscription::none(),
        }
    }
}

static ASSET_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/assets.zip"));

mod image_default {
    use std::{io::Read, sync::LazyLock};

    use zip::*;

    use crate::gui::game::ASSET_DATA;

    #[derive(Debug)]
    enum CacheError {
        NotFound,
        IoError(std::io::Error),
        ZipError(result::ZipError),
    }

    fn get_data_from_cache(path: &std::path::Path) -> Result<Vec<u8>, CacheError> {
        let cached_asset_file = crate::Application::app_dirs()
            .cache_dir()
            .to_path_buf()
            .join("assets")
            .join(path);
        if !cached_asset_file.exists() {
            return Err(CacheError::NotFound);
        }
        let mut buffer = Vec::new();
        let mut asset_file =
            std::fs::File::open(&cached_asset_file).map_err(CacheError::IoError)?;
        asset_file
            .read_to_end(&mut buffer)
            .map_err(CacheError::IoError)?;
        Ok(buffer)
    }

    fn create_cache() -> Result<(), CacheError> {
        let cached_asset_dir = crate::Application::app_dirs()
            .cache_dir()
            .to_path_buf()
            .join("assets");
        if !cached_asset_dir.exists() {
            std::fs::create_dir_all(&cached_asset_dir).map_err(CacheError::IoError)?;
        }
        let archive_data = std::io::Cursor::new(ASSET_DATA);
        let mut archive = ZipArchive::new(archive_data).map_err(CacheError::ZipError)?;
        archive
            .extract_unwrapped_root_dir(cached_asset_dir, zip::read::root_dir_common_filter)
            .map_err(CacheError::ZipError)?;
        Ok(())
    }

    pub(super) static OPENED_CELL: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("default/OpenedCell.png"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("default/OpenedCell.png"))
                    .expect("Failed to read OpenedCell from cache")
            }
            Err(e) => panic!("Failed to read OpenedCell from cache: {e:?}"),
        }
    });
    pub(super) static UNOPENED_CELL: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("default/UnopenedCell.png"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("default/UnopenedCell.png"))
                    .expect("Failed to read UnopenedCell from cache")
            }
            Err(e) => panic!("Failed to read UnopenedCell from cache: {e:?}"),
        }
    });
    pub(super) static MINE: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("default/Mine.png"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("default/Mine.png"))
                    .expect("Failed to read Mine from cache")
            }
            Err(e) => panic!("Failed to read Mine from cache: {e:?}"),
        }
    });
    pub(super) static FLAG: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let cache_result = get_data_from_cache(std::path::Path::new("default/Flag.png"));
        match cache_result {
            Ok(data) => data,
            Err(CacheError::NotFound) => {
                if let Err(e) = create_cache() {
                    panic!("Failed to create cache: {e:?}");
                }
                get_data_from_cache(std::path::Path::new("default/Flag.png"))
                    .expect("Failed to read Flag from cache")
            }
            Err(e) => panic!("Failed to read Flag from cache: {e:?}"),
        }
    });
}

impl Game {
    fn top_menu(&self) -> impl Into<Element<'_, Action>> {
        let remaining_mines = GuiWidget::text!("{}", self.board.get_remaining_mines());
        let reset_button = GuiWidget::button(":)").on_press(Action::ResetGame);
        let time_elapsed = (self.current_time - self.start_time).as_secs();
        let timer = if time_elapsed < 60 {
            GuiWidget::text!("{time_elapsed}").font(Font::MONOSPACE)
        } else if time_elapsed < (99 * 60) + 59 {
            GuiWidget::text!(
                "{minutes}:{seconds:02}",
                minutes = time_elapsed.div_euclid(60),
                seconds = time_elapsed.rem_euclid(60)
            )
            .font(Font::MONOSPACE)
        } else {
            GuiWidget::text("99:59").font(Font::MONOSPACE)
        };
        let content = GuiWidget::row![
            GuiWidget::container(remaining_mines).width(iced::Fill),
            GuiWidget::center_x(reset_button),
            GuiWidget::right(timer)
        ];
        content.width((self.board.get_width() as usize * 16) as f32)
    }
    fn cell_view(cell: &Cell) -> impl Into<Element<'_, Action>> {
        #[inline]
        fn cell_container<'a>(element: impl Into<Element<'a, Action>>) -> Element<'a, Action> {
            GuiWidget::center(element).width(16).height(16).into()
        }
        let mut stack = GuiWidget::Stack::with_capacity(2).height(16).width(16);
        if cell.is_open() {
            let open_image = GuiImage::Image::new(GuiImage::Handle::from_bytes(
                image_default::OPENED_CELL.as_slice(),
            ));
            stack = stack.push(open_image);
            if cell.is_mine() {
                let mine_image = cell_container(GuiImage::Image::new(
                    GuiImage::Handle::from_bytes(image_default::MINE.as_slice()),
                ));
                stack = stack.push(mine_image);
            } else if let Some(adjacent_mines) = cell.adjacent_mines()
                && adjacent_mines > 0
            {
                let color = match adjacent_mines {
                    1 => iced::color!(0, 0, 255),
                    2 => iced::color!(0, 127, 0),
                    3 => iced::color!(255, 0, 0),
                    4 => iced::color!(0, 0, 127),
                    5 => iced::color!(127, 0, 0),
                    6 => iced::color!(0, 127, 127),
                    7 => iced::color!(255, 255, 255),
                    8 => iced::color!(127, 127, 127),
                    // SAFETY: The internal enum AdjacentMines (which is what cell.adjacent_mines
                    // converts from) CANNOT represent values outside 0-8, and we
                    // just checked that adjacent_mines is not 0, so all other values are
                    // unreachable.
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };
                let text = GuiWidget::text!("{adjacent_mines}")
                    .font(Font::MONOSPACE)
                    .size(14)
                    .color(color);
                stack = stack.push(cell_container(text));
            }
        } else {
            let unopened_image = GuiImage::Image::new(GuiImage::Handle::from_bytes(
                image_default::UNOPENED_CELL.as_slice(),
            ));
            stack = stack.push(unopened_image);
            if cell.is_flagged() {
                let flag_image = cell_container(GuiImage::Image::new(
                    GuiImage::Handle::from_bytes(image_default::FLAG.as_slice()),
                ));
                stack = stack.push(flag_image);
            }
        }
        stack.width(16).height(16)
    }
    fn cell(&self, x: u8, y: u8) -> Element<'_, Action> {
        let cell = self.board.get_cell(x, y).expect("Cell should exist");
        let cell_view = Game::cell_view(cell);
        let is_playing = matches!(self.board.get_state(), BoardState::InProgress);
        if is_playing {
            GuiWidget::mouse_area(cell_view)
                .on_press(Action::OpenCell(x, y))
                .on_right_press(Action::ToggleFlag(x, y))
                .on_middle_press(Action::ChordCell(x, y))
                .into()
        } else {
            cell_view.into()
        }
    }
}
