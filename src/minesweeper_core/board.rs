use std::{
    collections::HashSet,
    num::{NonZeroU16, NonZeroU8, NonZeroUsize},
    ops::{Div, Rem},
};

use rand::prelude::*;

use crate::minesweeper_core::tile::*;

pub struct Board {
    width: NonZeroU8,
    height: NonZeroU8,
    mine_count: NonZeroU16,
    placed_mines: bool,
    tiles: Vec<Tile>,
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
        for _ in 0..height.get() * width.get() {
            tiles.push(Tile::default());
        }
        Ok(Self {
            width,
            height,
            mine_count,
            placed_mines: false,
            tiles,
        })
    }
    const fn coordinate_to_index(&self, x: u8, y: u8) -> Option<usize> {
        if x >= self.width.get() || y >= self.height.get() {
            return None;
        }
        Some((y as usize * self.width.get() as usize) + x as usize)
    }
    fn index_to_coordinate(&self, index: usize) -> Option<(u8, u8)> {
        if index >= self.width.get() as usize * self.height.get() as usize {
            return None;
        }
        Some((
            index.rem(NonZeroUsize::from(self.width)).try_into().ok()?,
            index.div(NonZeroUsize::from(self.width)).try_into().ok()?,
        ))
    }
    pub fn get(&self, x: u8, y: u8) -> Option<&Tile> {
        Some(&self.tiles[self.coordinate_to_index(x, y)?])
    }
    pub fn get_mut(&mut self, x: u8, y: u8) -> Option<&mut Tile> {
        let index = self.coordinate_to_index(x, y)?;
        Some(&mut self.tiles[index])
    }
    pub fn width(&self) -> u8 {
        self.width.get()
    }
    pub fn height(&self) -> u8 {
        self.height.get()
    }
    pub fn area(&self) -> u16 {
        self.width.get() as u16 * self.height.get() as u16
    }
    pub fn mine_count(&self) -> u16 {
        self.mine_count.get()
    }
    fn neighbor_coords(&self, x: u8, y: u8) -> Vec<(u8, u8)> {
        if x >= self.width.get() || y >= self.height.get() {
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
    pub fn check_tile(&mut self, x: u8, y: u8) {
        if let Some(tile) = self.get(x, y) {
            if tile.is_flagged() || tile.is_open() {
                return;
            }
        } else {
            return;
        }
        if !self.placed_mines {
            self.place_mines(x, y);
            self.placed_mines = true;
        }
        let tile = self.get_mut(x, y).unwrap();
        tile.open();
        if !tile.is_mined() {
            if tile.surrounding_mines().unwrap() == 0 {
                let neighbors = self.neighbor_coords(x, y);
                for neighbor in neighbors {
                    self.check_tile(neighbor.0, neighbor.1);
                }
            }
        } else {
            // todo save losing state
        }
    }
    fn place_mines(&mut self, x: u8, y: u8) {
        let neighbors = self.neighbor_coords(x, y);
        let mut all_tiles = Vec::with_capacity(self.area() as usize);
        for y in 0..self.height() {
            for x in 0..self.width() {
                all_tiles.push((x, y))
            }
        }
        let mut rng = SmallRng::from_entropy();
        let mines = if self.mine_count.get() <= self.area() - (neighbors.len() + 1) as u16 {
            let neighbors = neighbors.into_iter().collect::<HashSet<(u8, u8)>>();
            all_tiles
                .iter()
                .filter(|(tile_x, tile_y)| {
                    !neighbors.contains(&(*tile_x, *tile_y)) && (*tile_x, *tile_y) != (x, y)
                })
                .copied()
                .choose_multiple(&mut rng, self.mine_count.get() as usize)
        } else {
            all_tiles
                .iter()
                .filter(|(tile_x, tile_y)| (*tile_x, *tile_y) != (x, y))
                .copied()
                .choose_multiple(&mut rng, self.mine_count.get() as usize)
        };
        assert_eq!(mines.len(), self.mine_count.get() as usize);
        let mut increment_tiles = Vec::with_capacity(mines.len() * 8);
        for (x_coord, y_coord) in mines {
            self.get_mut(x_coord, y_coord).unwrap().become_mined();
            increment_tiles.append(self.neighbor_coords(x_coord, y_coord).as_mut());
        }
        for (x_coord, y_coord) in increment_tiles {
            self.get_mut(x_coord, y_coord)
                .unwrap()
                .increment_surrounding();
        }
        assert!(!self.get(x, y).unwrap().is_mined());
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
}
