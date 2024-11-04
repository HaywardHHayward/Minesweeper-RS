use std::{
    collections::HashSet,
    num::{NonZeroU16, NonZeroU8},
};

use crate::minesweeper_core::tile::*;

enum BoardState {
    FirstOpen,
    Playing,
    GameOver { hit_mine: bool },
}

pub struct Board {
    width: NonZeroU8,
    height: NonZeroU8,
    mine_count: NonZeroU16,
    tiles: Vec<Tile>,
    state: BoardState,
    unopened_tiles: HashSet<(u8, u8)>,
    mined_tiles: HashSet<(u8, u8)>,
}

#[derive(Debug)]
pub enum BuildError {
    ZeroedField,
    MineCountOverflow,
}

impl Board {
    pub fn build(width: u8, height: u8, mine_count: u16) -> Result<Self, BuildError> {
        if mine_count >= width as u16 * height as u16 {
            return Err(BuildError::MineCountOverflow);
        }
        let width = NonZeroU8::new(width).ok_or(BuildError::ZeroedField)?;
        let height = NonZeroU8::new(height).ok_or(BuildError::ZeroedField)?;
        let mine_count = NonZeroU16::new(mine_count).ok_or(BuildError::ZeroedField)?;
        let mut tiles = Vec::with_capacity(width.get() as usize * height.get() as usize);
        let mined_tiles = HashSet::with_capacity(mine_count.get() as usize);
        let mut unopened_tiles =
            HashSet::with_capacity(width.get() as usize * height.get() as usize);
        for y in 0..height.get() {
            for x in 0..width.get() {
                tiles.push(Tile::default());
                unopened_tiles.insert((x, y));
            }
        }
        Ok(Self {
            width,
            height,
            mine_count,
            tiles,
            state: BoardState::FirstOpen,
            unopened_tiles,
            mined_tiles,
        })
    }
    const fn coordinate_to_index(&self, x: u8, y: u8) -> Option<usize> {
        if x >= self.width() || y >= self.height() {
            return None;
        }
        Some((y as usize * self.width() as usize) + x as usize)
    }
    pub fn get(&self, x: u8, y: u8) -> Option<&Tile> {
        self.tiles.get(self.coordinate_to_index(x, y)?)
    }
    pub fn get_mut(&mut self, x: u8, y: u8) -> Option<&mut Tile> {
        let index = self.coordinate_to_index(x, y)?;
        self.tiles.get_mut(index)
    }
    pub const fn width(&self) -> u8 {
        self.width.get()
    }
    pub const fn height(&self) -> u8 {
        self.height.get()
    }
    pub const fn area(&self) -> u16 {
        self.width() as u16 * self.height() as u16
    }
    pub const fn mine_count(&self) -> u16 {
        self.mine_count.get()
    }
    pub fn flag_count(&self) -> usize {
        self.unopened_tiles
            .iter()
            .filter(|&coord| self.get(coord.0, coord.1).unwrap().is_flagged())
            .count()
    }
    fn neighbor_coords(&self, x: u8, y: u8) -> Vec<(u8, u8)> {
        if x >= self.width() || y >= self.height() {
            return Vec::new();
        }
        let mut neighbors = Vec::with_capacity(8);
        for y_offset in -1..=1 {
            let y_new = match y.overflowing_add_signed(y_offset) {
                (_, true) => continue,
                (added, false) if added >= self.height() => continue,
                (added, false) => added,
            };
            for x_offset in -1..=1 {
                if y_offset == 0 && x_offset == 0 {
                    continue;
                }
                let x_new = match x.overflowing_add_signed(x_offset) {
                    (_, true) => continue,
                    (added, false) if added >= self.width() => continue,
                    (added, false) => added,
                };
                neighbors.push((x_new, y_new))
            }
        }
        neighbors
    }
    pub fn open_tile(&mut self, x: u8, y: u8) {
        match self.get(x, y) {
            Some(tile) => {
                if tile.is_flagged() || tile.is_open() {
                    return;
                }
            }
            None => return,
        }
        if matches!(self.state, BoardState::FirstOpen) {
            self.place_mines(x, y);
            self.state = BoardState::Playing;
        }
        self.unopened_tiles.remove(&(x, y));
        let tile = self.get_mut(x, y).unwrap();
        tile.open();
        if !tile.is_mined() {
            if tile.surrounding_mines().unwrap() == 0 {
                let neighbors = self.neighbor_coords(x, y);
                for neighbor in neighbors {
                    self.open_tile(neighbor.0, neighbor.1);
                }
            }
        } else {
            self.state = BoardState::GameOver { hit_mine: true };
            return;
        }
        if self.unopened_tiles == self.mined_tiles {
            self.state = BoardState::GameOver { hit_mine: false };
        }
    }
    fn place_mines(&mut self, x: u8, y: u8) {
        use rand::prelude::*;
        let neighbors = self.neighbor_coords(x, y);
        let mut all_tiles = Vec::with_capacity(self.area() as usize);
        for y in 0..self.height() {
            for x in 0..self.width() {
                all_tiles.push((x, y))
            }
        }
        let mut rng = SmallRng::from_entropy();
        let mines = if self.mine_count() <= self.area() - (neighbors.len() + 1) as u16 {
            let neighbors = neighbors.into_iter().collect::<HashSet<(u8, u8)>>();
            all_tiles
                .iter()
                .filter(|(tile_x, tile_y)| {
                    !neighbors.contains(&(*tile_x, *tile_y)) && (*tile_x, *tile_y) != (x, y)
                })
                .copied()
                .choose_multiple(&mut rng, self.mine_count() as usize)
        } else {
            all_tiles
                .iter()
                .filter(|(tile_x, tile_y)| (*tile_x, *tile_y) != (x, y))
                .copied()
                .choose_multiple(&mut rng, self.mine_count() as usize)
        };
        let mut increment_tiles = Vec::with_capacity(mines.len() * 8);
        for (x_coord, y_coord) in mines {
            self.get_mut(x_coord, y_coord).unwrap().become_mined();
            self.mined_tiles.insert((x_coord, y_coord));
            increment_tiles.append(self.neighbor_coords(x_coord, y_coord).as_mut());
        }
        for (x_coord, y_coord) in increment_tiles {
            self.get_mut(x_coord, y_coord)
                .unwrap()
                .increment_surrounding();
        }
    }
    pub fn open_safe(&mut self, x: u8, y: u8) {
        match self.get(x, y) {
            Some(tile) => {
                if !tile.is_open() {
                    return;
                }
                assert!(!tile.is_mined());
                if tile.surrounding_mines().unwrap() == 0 {
                    return;
                }
            }
            None => return,
        }
        let (flagged, unflagged): (Vec<_>, Vec<_>) = self
            .neighbor_coords(x, y)
            .into_iter()
            .filter(|(tile_x, tile_y)| !self.get(*tile_x, *tile_y).unwrap().is_open())
            .partition(|(tile_x, tile_y)| self.get(*tile_x, *tile_y).unwrap().is_flagged());
        if flagged.len() != self.get(x, y).unwrap().surrounding_mines().unwrap() as usize {
            return;
        }
        for (tile_x, tile_y) in unflagged {
            self.open_tile(tile_x, tile_y);
        }
    }
    pub fn toggle_flag(&mut self, x: u8, y: u8) {
        let Some(tile) = self.get_mut(x, y) else {
            return;
        };
        tile.toggle_flag();
    }
    pub const fn hit_mine(&self) -> bool {
        matches!(self.state, BoardState::GameOver { hit_mine: true })
    }

    pub const fn is_playing(&self) -> bool {
        !matches!(self.state, BoardState::GameOver { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build() {
        for x in 1..16 {
            for y in 1..16 {
                for mine in 1..x as u16 * y as u16 {
                    assert!(Board::build(x, y, mine).is_ok());
                    assert!(Board::build(0, y, mine).is_err());
                    assert!(Board::build(x, 0, mine).is_err());
                    assert!(Board::build(x, y, 0).is_err());
                }
            }
        }
    }

    #[test]
    fn test_getters() {
        for x in 1..16 {
            for y in 1..16 {
                for mine in 1..x as u16 * y as u16 {
                    let board = Board::build(x, y, mine).unwrap();
                    assert_eq!(board.width(), x);
                    assert_eq!(board.height(), y);
                    assert_eq!(board.mine_count(), mine);
                }
            }
        }
    }

    #[test]
    fn test_mine_placement() {
        use rand::prelude::*;
        for x in 1..16 {
            for y in 1..16 {
                for mine in 1..x as u16 * y as u16 {
                    let mut board = Board::build(x, y, mine).unwrap();
                    let x_distribution = rand::distributions::Uniform::new(0, board.width());
                    let y_distribution = rand::distributions::Uniform::new(0, board.height());
                    let rand_x = x_distribution.sample(&mut thread_rng());
                    let rand_y = y_distribution.sample(&mut thread_rng());
                    board.place_mines(rand_x, rand_y);
                    assert_eq!(board.mined_tiles.len(), mine as usize);
                    assert!(!board.get(rand_x, rand_y).unwrap().is_mined());
                    let neighbors = board.neighbor_coords(rand_x, rand_y);
                    if mine <= (x as u16 * y as u16).saturating_sub(neighbors.len() as u16 + 1) {
                        for coord in neighbors {
                            assert!(!board.get(coord.0, coord.1).unwrap().is_mined());
                        }
                    }
                }
            }
        }
    }
}
