#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum SurroundingMines {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
}

pub enum SurroundingMinesConversionError {
    InvalidValue(u8),
}

enum MinedState {
    Safe { surrounding_mines: SurroundingMines },
    Mined,
}

enum TileState {
    Unopened { is_flagged: bool },
    Opened,
}

pub struct Tile {
    mine_state: MinedState,
    tile_state: TileState,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            mine_state: MinedState::Safe {
                surrounding_mines: SurroundingMines::Zero,
            },
            tile_state: TileState::Unopened { is_flagged: false },
        }
    }
}

enum Transition {
    ToggleFlag,
    BecomeMined,
    Open,
    IncrementSurroundingMines,
}

impl Tile {
    fn transition(&mut self, transition: Transition) {
        match transition {
            Transition::ToggleFlag => {
                if let TileState::Unopened { is_flagged } = self.tile_state {
                    self.tile_state = TileState::Unopened {
                        is_flagged: !is_flagged,
                    };
                }
            }
            Transition::BecomeMined => {
                if let TileState::Unopened { .. } = self.tile_state {
                    self.mine_state = MinedState::Mined;
                }
            }
            Transition::Open => {
                if let TileState::Unopened { is_flagged: false } = self.tile_state {
                    self.tile_state = TileState::Opened;
                }
            }
            Transition::IncrementSurroundingMines => {
                if let TileState::Unopened { .. } = self.tile_state {
                    if let MinedState::Safe { surrounding_mines } = self.mine_state {
                        self.mine_state = MinedState::Safe {
                            surrounding_mines: surrounding_mines.increment().unwrap(),
                        };
                    }
                }
            }
        }
    }
    pub const fn is_open(&self) -> bool {
        matches!(self.tile_state, TileState::Opened)
    }
    pub fn open(&mut self) {
        self.transition(Transition::Open);
    }
    pub const fn is_flagged(&self) -> bool {
        matches!(self.tile_state, TileState::Unopened { is_flagged: true })
    }
    pub fn toggle_flag(&mut self) {
        self.transition(Transition::ToggleFlag);
    }
    pub const fn is_mined(&self) -> bool {
        matches!(self.mine_state, MinedState::Mined)
    }
    pub fn become_mined(&mut self) {
        self.transition(Transition::BecomeMined);
    }
    pub const fn surrounding_mines(&self) -> Option<u8> {
        if let MinedState::Safe { surrounding_mines } = self.mine_state {
            Some(surrounding_mines as u8)
        } else {
            None
        }
    }
    pub fn increment_surrounding(&mut self) {
        self.transition(Transition::IncrementSurroundingMines);
    }
}

impl SurroundingMines {
    fn increment(self) -> Option<SurroundingMines> {
        SurroundingMines::try_from(u8::from(self) + 1).ok()
    }
}

impl From<SurroundingMines> for u8 {
    fn from(surrounding_mines: SurroundingMines) -> Self {
        surrounding_mines as u8
    }
}

impl TryFrom<u8> for SurroundingMines {
    type Error = SurroundingMinesConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 8 {
            return Err(SurroundingMinesConversionError::InvalidValue(value));
        }
        match value {
            0 => Ok(SurroundingMines::Zero),
            1 => Ok(SurroundingMines::One),
            2 => Ok(SurroundingMines::Two),
            3 => Ok(SurroundingMines::Three),
            4 => Ok(SurroundingMines::Four),
            5 => Ok(SurroundingMines::Five),
            6 => Ok(SurroundingMines::Six),
            7 => Ok(SurroundingMines::Seven),
            8 => Ok(SurroundingMines::Eight),
            _ => {
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let tile = Tile::default();
        assert!(matches!(
            tile.tile_state,
            TileState::Unopened { is_flagged: false }
        ));
        assert!(matches!(
            tile.mine_state,
            MinedState::Safe {
                surrounding_mines: SurroundingMines::Zero
            }
        ));
    }

    #[test]
    fn test_getters() {
        let default_tile = Tile::default();
        let mut mined_tile = Tile {
            tile_state: TileState::Unopened { is_flagged: false },
            mine_state: MinedState::Mined,
        };
        assert!(!default_tile.is_open());
        assert!(!mined_tile.is_open());
        assert!(!default_tile.is_flagged());
        assert!(!mined_tile.is_flagged());
        assert!(!default_tile.is_mined());
        assert!(mined_tile.is_mined());
        assert_eq!(
            default_tile
                .surrounding_mines()
                .expect("Should have surrounding mines"),
            0
        );
        assert!(mined_tile.surrounding_mines().is_none());
        let flagged_tile = Tile {
            tile_state: TileState::Unopened { is_flagged: true },
            mine_state: MinedState::Safe {
                surrounding_mines: SurroundingMines::One,
            },
        };
        mined_tile.tile_state = TileState::Unopened { is_flagged: true };
        assert!(!flagged_tile.is_open());
        assert!(!mined_tile.is_open());
        assert!(flagged_tile.is_flagged());
        assert!(mined_tile.is_flagged());
        assert!(!flagged_tile.is_mined());
        assert!(mined_tile.is_mined());
        assert_eq!(
            flagged_tile
                .surrounding_mines()
                .expect("Should have surrounding mines"),
            1
        );
        assert!(mined_tile.surrounding_mines().is_none());
        let opened_tile = Tile {
            tile_state: TileState::Opened,
            mine_state: MinedState::Safe {
                surrounding_mines: SurroundingMines::Two,
            },
        };
        mined_tile.tile_state = TileState::Opened;
        assert!(opened_tile.is_open());
        assert!(mined_tile.is_open());
        assert!(!opened_tile.is_flagged());
        assert!(!mined_tile.is_flagged());
        assert!(!opened_tile.is_mined());
        assert!(mined_tile.is_mined());
        assert_eq!(
            opened_tile
                .surrounding_mines()
                .expect("Should have surrounding mines"),
            2
        );
        assert!(mined_tile.surrounding_mines().is_none());
    }

    #[test]
    fn test_open() {
        let mut tile = Tile::default();
        assert!(!tile.is_open());
        tile.open();
        assert!(tile.is_open());
        let mut flagged_tile = Tile {
            mine_state: MinedState::Safe {
                surrounding_mines: SurroundingMines::Zero,
            },
            tile_state: TileState::Unopened { is_flagged: true },
        };
        assert!(!flagged_tile.is_open());
        flagged_tile.open();
        assert!(!flagged_tile.is_open());
    }

    #[test]
    fn test_toggle_flags() {
        let mut tile = Tile::default();
        assert!(!tile.is_flagged());
        tile.toggle_flag();
        assert!(tile.is_flagged());
        tile.toggle_flag();
        assert!(!tile.is_flagged());
        let mut opened_tile = Tile::default();
        opened_tile.open();
        assert!(!opened_tile.is_flagged());
        opened_tile.toggle_flag();
        assert!(!opened_tile.is_flagged());
    }

    #[test]
    fn test_become_mined() {
        let mut tile = Tile::default();
        let mut flagged_tile = Tile::default();
        flagged_tile.toggle_flag();
        let mut opened_tile = Tile::default();
        opened_tile.open();
        assert!(!tile.is_mined());
        assert!(!flagged_tile.is_mined());
        assert!(!opened_tile.is_mined());
        tile.become_mined();
        flagged_tile.become_mined();
        opened_tile.become_mined();
        assert!(tile.is_mined());
        assert!(flagged_tile.is_mined());
        assert!(!opened_tile.is_mined());
    }

    #[test]
    fn test_surrounding_mines() {
        let mut tile = Tile::default();
        let mut flagged_tile = Tile::default();
        flagged_tile.toggle_flag();
        let mut opened_tile = Tile::default();
        opened_tile.open();
        let mut mined_tile = Tile::default();
        mined_tile.become_mined();
        assert!(mined_tile.surrounding_mines().is_none());
        assert_eq!(tile.surrounding_mines(), Some(0));
        assert_eq!(flagged_tile.surrounding_mines(), Some(0));
        assert_eq!(opened_tile.surrounding_mines(), Some(0));
        for i in 1..8 {
            tile.increment_surrounding();
            flagged_tile.increment_surrounding();
            opened_tile.increment_surrounding();
            assert_eq!(tile.surrounding_mines(), Some(i));
            assert_eq!(flagged_tile.surrounding_mines(), Some(i));
            assert_eq!(opened_tile.surrounding_mines(), Some(0));
        }
    }
}
