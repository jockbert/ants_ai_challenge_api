/// Position on world map. Upper left corner is position (0,0), i.e column 0 and row 0.
#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
pub struct Position {
    /// Row in cartesian map coordinates, comparable to Y-axis value.
    pub row: u16,
    /// Column in cartesian map coordinates, comparable to X-axis value.
    pub col: u16,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Order {
    pub pos: Position,
    pub dir: Direction,
}

pub type Orders = Vec<Order>;

/// Helper function (short hand) for a Position.
pub fn pos(row: u16, column: u16) -> Position {
    Position {
        row: row,
        col: column,
    }
}

impl Position {
    pub fn order(&self, dir: Direction) -> Order {
        Order {
            pos: self.clone(),
            dir: dir,
        }
    }
}

impl Order {
    /// Get target position of order.
    ///
    /// Will wrap around in the 2D range from (0,0) inclusive
    /// to the given scope size exclusive.
    pub fn target_pos(&self, scope_size: &Position) -> Position {
        use self::Direction::*;
        let row_max = scope_size.row;
        let col_max = scope_size.col;

        match self {
            Order { pos: p, dir: South } => pos((p.row + 1) % row_max, p.col),
            Order { pos: p, dir: North } => pos((p.row + row_max - 1) % row_max, p.col),
            Order { pos: p, dir: West } => pos(p.row, (p.col + col_max - 1) % col_max),
            Order { pos: p, dir: East } => pos(p.row, (p.col + 1) % col_max),
        }
    }
}
