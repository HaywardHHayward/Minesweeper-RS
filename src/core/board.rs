use std::{
    num::{NonZeroU16, NonZeroU8},
    ops::{Index, IndexMut},
};

use rand::prelude::*;

use crate::core::cell::Cell;

#[derive(Debug)]
pub struct Board {
    cells: Vec<Cell>,
    width: NonZeroU8,
    height: NonZeroU8,
    mine_count: NonZeroU16,
    first_open: bool,
}

#[derive(Debug)]
pub enum BoardError {
    InvalidBoardSize,
    TooManyMines(NonZeroU16),
}

impl Board {
    pub fn create(
        width: NonZeroU8,
        height: NonZeroU8,
        mine_count: NonZeroU16,
    ) -> Result<Self, BoardError> {
        if width.get() == 1 && height.get() == 1 {
            return Err(BoardError::InvalidBoardSize);
        }
        let board_area = (width.get() as u16) * (height.get() as u16);
        if mine_count.get() >= board_area {
            return Err(BoardError::TooManyMines(mine_count));
        }
        let mut cells = Vec::with_capacity(board_area as usize);
        for _ in 0..board_area {
            let cell = Cell::new();
            cells.push(cell);
        }
        Ok(Self {
            cells,
            width,
            height,
            mine_count,
            first_open: true,
        })
    }
    pub fn get_cell(&self, x: u8, y: u8) -> Option<&Cell> {
        if x >= self.get_width() || y >= self.get_height() {
            return None;
        }
        let index = coordinate_to_linear(x, y, self.width);
        self.cells.get(index)
    }
    pub fn get_cell_mut(&mut self, x: u8, y: u8) -> Option<&mut Cell> {
        if x >= self.get_width() || y >= self.get_height() {
            return None;
        }
        let index = coordinate_to_linear(x, y, self.width);
        self.cells.get_mut(index)
    }
    pub const fn get_width(&self) -> u8 {
        self.width.get()
    }
    pub const fn get_height(&self) -> u8 {
        self.height.get()
    }
    pub fn open_cell(&mut self, x: u8, y: u8) {
        {
            let Some(cell) = self.get_cell(x, y) else {
                return;
            };
            if cell.is_open() || cell.is_flagged() {
                return;
            }
        }
        if self.first_open {
            self.generate_mines(x, y);
            self.first_open = false;
        }
        let cell = self.get_cell_mut(x, y).unwrap();
        cell.open();
        if cell.is_mine() {
            return;
        }
        if cell.adjacent_mines().unwrap() == 0 {
            for (surrounding_x, surrounding_y) in self.get_surrounding_coordinates(x, y) {
                self.open_cell(surrounding_x, surrounding_y);
            }
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
    fn get_surrounding_coordinates(&self, x: u8, y: u8) -> Vec<(u8, u8)> {
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
        coordinates
    }
    fn generate_mines(&mut self, x: u8, y: u8) {
        let mut rng = SmallRng::from_os_rng();
        let total_area = self.get_width() as u16 * self.get_height() as u16;
        let mut surrounding_coordinates = self.get_surrounding_coordinates(x, y);
        let too_many_mines =
            total_area - (surrounding_coordinates.len() as u16 + 1) < self.mine_count.get();
        let mut disallowed_coordinates: Vec<(u8, u8)> = vec![(x, y)];
        if !too_many_mines {
            disallowed_coordinates.append(&mut surrounding_coordinates);
        }
        let mut full_coordinates = Vec::with_capacity(total_area as usize);
        for cell_x in 0..self.get_width() {
            for cell_y in 0..self.get_height() {
                if disallowed_coordinates.contains(&(cell_x, cell_y)) {
                    continue;
                }
                full_coordinates.push((cell_x, cell_y));
            }
        }
        let mined_coordinates =
            full_coordinates.choose_multiple(&mut rng, self.mine_count.get() as usize);
        for (mine_x, mine_y) in mined_coordinates {
            for (cell_x, cell_y) in self.get_surrounding_coordinates(*mine_x, *mine_y) {
                self.get_cell_mut(cell_x, cell_y)
                    .unwrap()
                    .increment_adjacent_mines();
            }
            self.get_cell_mut(*mine_x, *mine_y).unwrap().become_mined();
        }
    }
}

impl Index<(u8, u8)> for Board {
    type Output = Cell;
    fn index(&self, index: (u8, u8)) -> &Self::Output {
        self.get_cell(index.0, index.1)
            .expect("Index out of bounds")
    }
}

impl IndexMut<(u8, u8)> for Board {
    fn index_mut(&mut self, index: (u8, u8)) -> &mut Self::Output {
        self.get_cell_mut(index.0, index.1)
            .expect("Index out of bounds")
    }
}

const fn coordinate_to_linear(x: u8, y: u8, width: NonZeroU8) -> usize {
    y as usize * width.get() as usize + x as usize
}

#[cfg(test)]
mod testing {
    use super::*;
    fn create_board(x: u8, y: u8, m: u16) -> Result<Board, BoardError> {
        Board::create(
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
    }
    #[test]
    fn test_board_get_cell() {
        let board = create_board(5, 5, 5).unwrap();
        assert!(board.get_cell(5, 5).is_none());
        assert!(board.get_cell(4, 5).is_none());
        assert!(board.get_cell(5, 4).is_none());
        assert!(board.get_cell(4, 4).is_some());
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
}
