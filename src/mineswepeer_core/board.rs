use std::{
    num::{NonZero, NonZeroU16, NonZeroU8, NonZeroUsize},
    ops::{Div, Rem},
};

use crate::mineswepeer_core::tile::*;

pub struct Board {
    width: NonZeroU8,
    height: NonZeroU8,
    mine_count: NonZeroU16,
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
            tiles,
        })
    }
    fn coordinate_to_index(&self, x: u8, y: u8) -> Option<usize> {
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
    pub fn mine_count(&self) -> u16 {
        self.mine_count.get()
    }
}
