use std::{
    collections::HashSet,
    num::{NonZeroU8, NonZeroU16},
};

use rand::prelude::*;

use crate::core::cell::Cell;

#[derive(Copy, Clone, Debug)]
pub enum BoardState {
    InProgress,
    Won,
    Lost,
}

#[derive(Debug)]
pub struct Board {
    cells: Vec<Cell>,
    width: NonZeroU8,
    height: NonZeroU8,
    mine_count: NonZeroU16,
    unopened_coordinates: HashSet<(u8, u8)>,
    mined_coordinates: HashSet<(u8, u8)>,
    first_open: bool,
    state: BoardState,
}

#[derive(Debug)]
pub enum BoardError {
    InvalidBoardSize,
    TooManyMines(NonZeroU16), // The number of mines equals to or exceeds the board area
}

impl Board {
    pub fn create_custom(
        width: NonZeroU8,
        height: NonZeroU8,
        mine_count: NonZeroU16,
    ) -> Result<Self, BoardError> {
        if width.get() == 1 && height.get() == 1 {
            // A 1x1 board either can have no mines (we need at least one mine, or it is not
            // a game) or be entirely filled with mines (which is not a game either)
            return Err(BoardError::InvalidBoardSize);
        }
        let board_area = (width.get() as u16) * (height.get() as u16);
        if mine_count.get() >= board_area {
            // The number of mines is equal to the board area (which is not a game) or
            // exceeds it (which is impossible to construct)
            return Err(BoardError::TooManyMines(
                NonZeroU16::new(board_area - 1).unwrap(),
            ));
        }
        let cells = vec![Cell::new(); board_area as usize];
        let unopened_coordinates = (0..width.get())
            .flat_map(|x| (0..height.get()).map(move |y| (x, y)))
            .collect();
        let mined_coordinates = HashSet::with_capacity(mine_count.get() as usize);
        Ok(Self {
            cells,
            width,
            height,
            mine_count,
            unopened_coordinates,
            mined_coordinates,
            first_open: true,
            state: BoardState::InProgress,
        })
    }
    pub fn create_beginner() -> Self {
        Self::create_custom(
            NonZeroU8::new(9).unwrap(),
            NonZeroU8::new(9).unwrap(),
            NonZeroU16::new(10).unwrap(),
        )
        .unwrap()
    }
    pub fn create_intermediate() -> Self {
        Self::create_custom(
            NonZeroU8::new(16).unwrap(),
            NonZeroU8::new(16).unwrap(),
            NonZeroU16::new(40).unwrap(),
        )
        .unwrap()
    }
    pub fn create_expert() -> Self {
        Self::create_custom(
            NonZeroU8::new(30).unwrap(),
            NonZeroU8::new(16).unwrap(),
            NonZeroU16::new(99).unwrap(),
        )
        .unwrap()
    }
    pub fn get_cell(&self, x: u8, y: u8) -> Option<&Cell> {
        if x >= self.get_width() || y >= self.get_height() {
            return None;
        }
        self.cells.get(coordinate_to_linear(x, y, self.width))
    }
    pub fn get_cell_mut(&mut self, x: u8, y: u8) -> Option<&mut Cell> {
        if x >= self.get_width() || y >= self.get_height() {
            return None;
        }
        self.cells.get_mut(coordinate_to_linear(x, y, self.width))
    }
    pub const fn get_width(&self) -> u8 {
        self.width.get()
    }
    pub const fn get_height(&self) -> u8 {
        self.height.get()
    }
    pub const fn get_mine_count(&self) -> u16 {
        self.mine_count.get()
    }
    pub fn get_remaining_mines(&self) -> i32 {
        // Subtracts how many cells have been flagged from how many mines there are
        (self.mine_count.get() as i32)
            - (self
                .unopened_coordinates
                .iter()
                .filter(|(x, y)| self.get_cell(*x, *y).unwrap().is_flagged())
                .count() as i32)
    }
    pub fn open_cell(&mut self, x: u8, y: u8) {
        if !self.unopened_coordinates.contains(&(x, y)) {
            return;
        }
        if self.first_open {
            // Wait for the first cell to be opened before generating mines, since this
            // prevents the first cell to be opened on a mine, and thus losing the game
            // immediately.
            self.generate_mines(x, y);
            self.first_open = false;
        }
        let cell = self.get_cell(x, y).unwrap();
        if cell.is_flagged() {
            return;
        }
        self.unopened_coordinates.remove(&(x, y));
        let cell = self.get_cell_mut(x, y).unwrap();
        cell.open();
        if cell.is_mine() {
            self.state = BoardState::Lost;
            return;
        }
        if cell.adjacent_mines().unwrap() == 0 {
            // At some point I will try and optimize this via multithreading like in my C++
            // version, but for now I will just open all the surrounding cells
            // consecutively.
            for (surrounding_x, surrounding_y) in self.get_surrounding_coordinates(x, y) {
                self.open_cell(surrounding_x, surrounding_y);
            }
        }
        if self.unopened_coordinates == self.mined_coordinates {
            self.state = BoardState::Won;
        }
    }
    pub fn chord_cell(&mut self, x: u8, y: u8) {
        let Some(cell) = self.get_cell(x, y) else {
            return;
        };
        if !cell.is_open() {
            return;
        }
        let surrounding_coordinates = self.get_surrounding_coordinates(x, y);
        // Get rid of any surrounding cells that are already opened (if they're open
        // then they're obviously safe otherwise the game would be over), and then
        // partition them into whether they are flagged or not.
        let (flagged, unflagged): (Vec<_>, Vec<_>) = surrounding_coordinates
            .filter(|(x, y)| self.get_cell(*x, *y).is_some_and(|cell| !cell.is_open()))
            .partition(|(x, y)| self.get_cell(*x, *y).is_some_and(|cell| cell.is_flagged()));
        // If the number of flagged cells does not match the number of adjacent mines,
        // then we do not meet the requirements for opening the unflagged cells.
        if flagged.len() != cell.adjacent_mines().unwrap() as usize {
            return;
        }
        // If we reach this point, then we open all the unflagged cells, since the
        // number of flagged cells is equal to the number of adjacent mines.
        for (surrounding_x, surrounding_y) in unflagged {
            self.open_cell(surrounding_x, surrounding_y);
        }
    }
    pub fn toggle_flag(&mut self, x: u8, y: u8) {
        let Some(cell) = self.get_cell_mut(x, y) else {
            return;
        };
        if cell.is_open() {
            return;
        }
        cell.toggle_flag();
    }
    pub fn get_state(&self) -> BoardState {
        self.state
    }
    fn get_surrounding_coordinates(&self, x: u8, y: u8) -> impl Iterator<Item = (u8, u8)> + use<> {
        let mut coordinates = Vec::with_capacity(8);
        for x_new in x.saturating_sub(1)..=x.saturating_add(1) {
            if x_new >= self.get_width() {
                continue;
            }
            for y_new in y.saturating_sub(1)..=y.saturating_add(1) {
                if y_new >= self.get_height() {
                    continue;
                }
                if (x_new == x) && (y_new == y) {
                    continue;
                }
                coordinates.push((x_new, y_new));
            }
        }
        coordinates.into_iter()
    }
    fn generate_mines(&mut self, x: u8, y: u8) {
        let mut rng = SmallRng::from_os_rng();
        let total_area = self.get_width() as u16 * self.get_height() as u16;
        let mut surrounding_coordinates =
            self.get_surrounding_coordinates(x, y).collect::<Vec<_>>();
        // In order to make the game more fun, I try and avoid placing mines in the
        // cells surrounding the cell selected. However, if the number of mines is too
        // high, this may not be possible, and therefore must be accounted for.
        let too_many_mines =
            total_area - (surrounding_coordinates.len() as u16 + 1) < self.mine_count.get();
        let mut disallowed_coordinates: Vec<(u8, u8)> = vec![(x, y)];
        // If we have enough space, we will not place mines in the cells surrounding the
        // cell specified
        if !too_many_mines {
            disallowed_coordinates.append(&mut surrounding_coordinates);
        }
        let mut possible_coordinates = Vec::with_capacity(total_area as usize);
        // Can't find a more clever way to do this, so I just iterate through all the
        // possible coordinates and do not push them into the possible coordinates if
        // they're disallowed. If you have a better idea, please let me know.
        for cell_x in 0..self.get_width() {
            for cell_y in 0..self.get_height() {
                if disallowed_coordinates.contains(&(cell_x, cell_y)) {
                    continue;
                }
                possible_coordinates.push((cell_x, cell_y));
            }
        }
        let mined_coordinates =
            possible_coordinates.choose_multiple(&mut rng, self.mine_count.get() as usize);
        for (mine_x, mine_y) in mined_coordinates {
            for (cell_x, cell_y) in self.get_surrounding_coordinates(*mine_x, *mine_y) {
                self.get_cell_mut(cell_x, cell_y)
                    .unwrap()
                    .increment_adjacent_mines();
            }
            self.get_cell_mut(*mine_x, *mine_y).unwrap().become_mined();
            self.mined_coordinates.insert((*mine_x, *mine_y));
        }
    }
}

const fn coordinate_to_linear(x: u8, y: u8, width: NonZeroU8) -> usize {
    y as usize * width.get() as usize + x as usize
}

#[cfg(test)]
mod testing {
    use super::*;
    fn create_board(x: u8, y: u8, m: u16) -> Result<Board, BoardError> {
        Board::create_custom(
            NonZeroU8::new(x).unwrap(),
            NonZeroU8::new(y).unwrap(),
            NonZeroU16::new(m).unwrap(),
        )
    }
    #[test]
    fn test_board_creation() {
        let board_check = create_board(5, 10, 15);
        assert!(board_check.is_ok());
        let board = board_check.unwrap();
        assert_eq!(board.get_width(), 5);
        assert_eq!(board.get_height(), 10);
        assert_eq!(board.cells.len(), 50);
        assert_eq!(board.mine_count.get(), 15);
        assert!(board.first_open);
    }
    #[test]
    fn test_board_creation_invalid() {
        let board_check = create_board(1, 1, 1);
        assert!(board_check.is_err());
        let board_check = create_board(5, 10, 50);
        assert!(board_check.is_err());
        let board_check = create_board(5, 10, 51);
        assert!(board_check.is_err());
    }
    #[test]
    fn test_board_get_cell() {
        let mut board = create_board(5, 5, 5).unwrap();
        assert!(board.get_cell(5, 5).is_none());
        assert!(board.get_cell(4, 5).is_none());
        assert!(board.get_cell(5, 4).is_none());
        assert!(board.get_cell(4, 4).is_some());
        assert!(board.get_cell_mut(5, 5).is_none());
        assert!(board.get_cell_mut(4, 5).is_none());
        assert!(board.get_cell_mut(5, 4).is_none());
        assert!(board.get_cell_mut(4, 4).is_some());
    }
    #[test]
    fn test_board_check_normal() {
        for x in 0..5 {
            for y in 0..5 {
                let mut board = create_board(5, 5, 16).unwrap();
                board.open_cell(x, y);
                let cell = board.get_cell(x, y).unwrap();
                assert!(cell.is_open());
                assert_eq!(cell.adjacent_mines().unwrap(), 0);
                for (sur_x, sur_y) in board.get_surrounding_coordinates(x, y) {
                    assert!(!board.get_cell(sur_x, sur_y).unwrap().is_mine());
                }
            }
        }
    }
    #[test]
    fn test_board_check_filled() {
        for x in 1..4 {
            for y in 1..4 {
                let mut board = create_board(5, 5, 24).unwrap();
                board.open_cell(x, y);
                let cell = board.get_cell(x, y).unwrap();
                assert!(cell.is_open());
                assert_eq!(cell.adjacent_mines().unwrap(), 8);
                for (sur_x, sur_y) in board.get_surrounding_coordinates(x, y) {
                    assert!(board.get_cell(sur_x, sur_y).unwrap().is_mine());
                }
            }
        }
    }
    #[test]
    fn test_board_toggle_flag() {
        let mut board = create_board(5, 5, 5).unwrap();
        board.toggle_flag(0, 0);
        assert!(board.get_cell(0, 0).unwrap().is_flagged());
        board.toggle_flag(0, 0);
        assert!(!board.get_cell(0, 0).unwrap().is_flagged());
    }
    #[test]
    fn test_board_no_op() {
        let mut board = create_board(5, 5, 5).unwrap();
        board.toggle_flag(0, 0);
        board.toggle_flag(0, 0);
        assert!(!board.get_cell(0, 0).unwrap().is_flagged());
        board.open_cell(0, 0);
        assert!(board.get_cell(0, 0).unwrap().is_open());
        let pre_double_check = board.unopened_coordinates.clone();
        board.toggle_flag(0, 0);
        assert!(!board.get_cell(0, 0).unwrap().is_flagged());
        board.open_cell(0, 0);
        assert!(board.get_cell(0, 0).unwrap().is_open());
        assert_eq!(board.unopened_coordinates, pre_double_check);
    }
}
