use std::ops::{Add, AddAssign};

#[derive(Clone, Debug)]
pub struct Cell {
    open_state: OpenState,
    mine_state: MineState,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum AdjacentMines {
    #[default]
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

impl Add<u8> for AdjacentMines {
    type Output = Option<AdjacentMines>;

    fn add(self, rhs: u8) -> Self::Output {
        let sum = u8::from(self) + rhs;
        AdjacentMines::try_from(sum).ok()
    }
}

impl AddAssign<u8> for AdjacentMines {
    fn add_assign(&mut self, rhs: u8) {
        if let Ok(new_value) = AdjacentMines::try_from(u8::from(*self) + rhs) {
            *self = new_value;
        }
    }
}

impl From<AdjacentMines> for u8 {
    fn from(adjacent_mines: AdjacentMines) -> u8 {
        adjacent_mines as u8
    }
}

impl TryFrom<u8> for AdjacentMines {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AdjacentMines::Zero),
            1 => Ok(AdjacentMines::One),
            2 => Ok(AdjacentMines::Two),
            3 => Ok(AdjacentMines::Three),
            4 => Ok(AdjacentMines::Four),
            5 => Ok(AdjacentMines::Five),
            6 => Ok(AdjacentMines::Six),
            7 => Ok(AdjacentMines::Seven),
            8 => Ok(AdjacentMines::Eight),
            invalid_num => Err(invalid_num),
        }
    }
}

impl std::fmt::Display for AdjacentMines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}

#[derive(Clone, Debug)]
enum OpenState {
    Opened,
    Unopened { is_flagged: bool },
}

#[derive(Clone, Debug)]
enum MineState {
    Mined,
    Safe { adjacent_mines: AdjacentMines },
}

#[derive(Debug)]
enum CellEvent {
    Open,
    ToggleFlag,
    BecomeMined,
    IncrementAdjacentMines,
}

impl Cell {
    pub const fn new() -> Self {
        Cell {
            open_state: OpenState::Unopened { is_flagged: false },
            mine_state: MineState::Safe {
                adjacent_mines: AdjacentMines::Zero,
            },
        }
    }
    pub fn open(&mut self) {
        self.cell_transition(CellEvent::Open);
    }
    pub const fn is_open(&self) -> bool {
        matches!(self.open_state, OpenState::Opened)
    }
    pub fn toggle_flag(&mut self) {
        self.cell_transition(CellEvent::ToggleFlag);
    }
    pub const fn is_flagged(&self) -> bool {
        matches!(self.open_state, OpenState::Unopened { is_flagged: true })
    }
    pub fn become_mined(&mut self) {
        self.cell_transition(CellEvent::BecomeMined);
    }
    pub const fn is_mine(&self) -> bool {
        matches!(self.mine_state, MineState::Mined)
    }
    pub fn increment_adjacent_mines(&mut self) {
        self.cell_transition(CellEvent::IncrementAdjacentMines);
    }
    pub fn adjacent_mines(&self) -> Option<AdjacentMines> {
        if let MineState::Safe { adjacent_mines } = self.mine_state {
            Some(adjacent_mines)
        } else {
            None
        }
    }
    // Single function to handle all cell state transitions
    fn cell_transition(&mut self, cell_event: CellEvent) {
        match (cell_event, &self.open_state, &self.mine_state) {
            // If the cell is already opened, we should not do anything to change its state
            (_, OpenState::Opened, _) => (),
            (CellEvent::Open, OpenState::Unopened { is_flagged: false }, _) => {
                self.open_state = OpenState::Opened;
            }
            (CellEvent::ToggleFlag, OpenState::Unopened { is_flagged }, _) => {
                self.open_state = OpenState::Unopened {
                    is_flagged: !is_flagged,
                };
            }
            (CellEvent::BecomeMined, OpenState::Unopened { .. }, MineState::Safe { .. }) => {
                self.mine_state = MineState::Mined;
            }
            (
                CellEvent::IncrementAdjacentMines,
                OpenState::Unopened { .. },
                MineState::Safe { adjacent_mines },
            ) => {
                // This should always succeed, but better to be safe and only increment if it's
                // less than 8
                if let Some(new_adjacent_mines) = *adjacent_mines + 1 {
                    self.mine_state = MineState::Safe {
                        adjacent_mines: new_adjacent_mines,
                    };
                }
            }
            (..) => (),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn test_cell_creation() {
        let cell = Cell::new();
        assert!(!cell.is_open());
        assert!(!cell.is_flagged());
        assert!(!cell.is_mine());
        assert_eq!(cell.adjacent_mines(), Some(AdjacentMines::Zero));
    }
    #[test]
    fn test_cell_open() {
        let mut cell = Cell::new();
        cell.open();
        assert!(cell.is_open());
    }
    #[test]
    fn test_cell_open_flagged() {
        let mut cell = Cell::new();
        cell.toggle_flag();
        cell.open();
        assert!(!cell.is_open());
    }
    #[test]
    fn test_cell_toggle_flag() {
        let mut cell = Cell::new();
        cell.toggle_flag();
        assert!(cell.is_flagged());
        cell.toggle_flag();
        assert!(!cell.is_flagged());
    }
    #[test]
    fn test_cell_toggle_flag_on_opened() {
        let mut cell = Cell::new();
        cell.open();
        cell.toggle_flag();
        assert!(!cell.is_flagged());
    }
    #[test]
    fn test_cell_become_mined() {
        let mut cell = Cell::new();
        cell.become_mined();
        assert!(cell.is_mine());
    }
    #[test]
    fn test_cell_become_mined_on_opened() {
        let mut cell = Cell::new();
        cell.open();
        cell.become_mined();
        assert!(!cell.is_mine());
    }
    #[test]
    fn test_cell_become_mined_on_flagged() {
        let mut cell = Cell::new();
        cell.toggle_flag();
        cell.become_mined();
        assert!(cell.is_mine());
    }
    #[test]
    fn test_cell_increment_adjacent_mines() {
        let mut cell = Cell::new();
        for num in 1..=8 {
            cell.increment_adjacent_mines();
            assert_eq!(
                cell.adjacent_mines(),
                Some(AdjacentMines::try_from(num).unwrap())
            );
        }
        cell.increment_adjacent_mines();
        assert_eq!(cell.adjacent_mines(), Some(AdjacentMines::Eight));
    }
    #[test]
    fn test_cell_increment_adjacent_mines_on_opened() {
        let mut cell = Cell::new();
        cell.open();
        cell.increment_adjacent_mines();
        assert_eq!(cell.adjacent_mines(), Some(AdjacentMines::Zero));
    }
    #[test]
    fn test_cell_increment_adjacent_mines_on_mined() {
        let mut cell = Cell::new();
        cell.become_mined();
        cell.increment_adjacent_mines();
        assert_eq!(cell.adjacent_mines(), None);
    }
}
