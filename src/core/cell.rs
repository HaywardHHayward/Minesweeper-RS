#[derive(Debug)]
pub(crate) struct Cell {
    open_state: OpenState,
    mine_state: MineState,
}

#[derive(Clone, Copy, Debug)]
enum AdjacentMines {
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

#[derive(Debug)]
enum OpenState {
    Opened,
    Unopened { is_flagged: bool },
}

#[derive(Debug)]
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
    pub(crate) const fn new() -> Self {
        Cell {
            open_state: OpenState::Unopened { is_flagged: false },
            mine_state: MineState::Safe {
                adjacent_mines: AdjacentMines::Zero,
            },
        }
    }
    fn cell_transition(&mut self, cell_event: CellEvent) {
        match cell_event {
            CellEvent::Open => {
                if matches!(self.open_state, OpenState::Unopened { is_flagged: false }) {
                    self.open_state = OpenState::Opened;
                }
            }
            CellEvent::ToggleFlag => {
                if let OpenState::Unopened { is_flagged } = &mut self.open_state {
                    *is_flagged = !*is_flagged;
                }
            }
            CellEvent::BecomeMined => {
                if matches!(self.open_state, OpenState::Opened) {
                    return;
                }
                self.mine_state = MineState::Mined;
            }
            CellEvent::IncrementAdjacentMines => {
                if matches!(self.open_state, OpenState::Opened) {
                    return;
                }
                if let MineState::Safe { adjacent_mines } = &mut self.mine_state {
                    if u8::from(*adjacent_mines) < 8 {
                        *adjacent_mines = AdjacentMines::try_from(u8::from(*adjacent_mines) + 1)
                            .expect("Invalid number of adjacent mines");
                    }
                }
            }
        }
    }
    pub(crate) fn open(&mut self) {
        self.cell_transition(CellEvent::Open);
    }
    pub(crate) const fn is_open(&self) -> bool {
        matches!(self.open_state, OpenState::Opened)
    }
    pub(crate) fn toggle_flag(&mut self) {
        self.cell_transition(CellEvent::ToggleFlag);
    }
    pub(crate) const fn is_flagged(&self) -> bool {
        matches!(self.open_state, OpenState::Unopened { is_flagged: true })
    }
    pub(crate) fn become_mined(&mut self) {
        self.cell_transition(CellEvent::BecomeMined);
    }
    pub(crate) const fn is_mine(&self) -> bool {
        matches!(self.mine_state, MineState::Mined)
    }
    pub(crate) fn increment_adjacent_mines(&mut self) {
        self.cell_transition(CellEvent::IncrementAdjacentMines);
    }
    pub(crate) fn adjacent_mines(&self) -> Option<u8> {
        if let MineState::Safe { adjacent_mines } = &self.mine_state {
            Some(u8::from(*adjacent_mines))
        } else {
            None
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
        assert_eq!(cell.adjacent_mines(), Some(0));
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
            assert_eq!(cell.adjacent_mines(), Some(num));
        }
        cell.increment_adjacent_mines();
        assert_eq!(cell.adjacent_mines(), Some(8));
    }
    #[test]
    fn test_cell_increment_adjacent_mines_on_opened() {
        let mut cell = Cell::new();
        cell.open();
        cell.increment_adjacent_mines();
        assert_eq!(cell.adjacent_mines(), Some(0));
    }
    #[test]
    fn test_cell_increment_adjacent_mines_on_mined() {
        let mut cell = Cell::new();
        cell.become_mined();
        cell.increment_adjacent_mines();
        assert_eq!(cell.adjacent_mines(), None);
    }
}
