use std::fmt;

/// Position on world map. Upper left corner is position (0,0), i.e column 0 and row 0.
#[derive(PartialEq, PartialOrd, Eq, Ord, Default, Hash, Clone)]
pub struct Position {
    /// Row in cartesian map coordinates, comparable to Y-axis value.
    pub row: u16,
    /// Column in cartesian map coordinates, comparable to X-axis value.
    pub col: u16,
}

impl Position {
    /// Short hand for order to the north direction.
    pub fn north(&self) -> Order {
        Order {
            pos: self.clone(),
            dir: Direction::North,
        }
    }
    /// Short hand for order to the south direction.
    pub fn south(&self) -> Order {
        Order {
            pos: self.clone(),
            dir: Direction::South,
        }
    }
    /// Short hand for order to the west direction.
    pub fn west(&self) -> Order {
        Order {
            pos: self.clone(),
            dir: Direction::West,
        }
    }
    /// Short hand for order to the east direction.
    pub fn east(&self) -> Order {
        Order {
            pos: self.clone(),
            dir: Direction::East,
        }
    }

    /// Shorthand for order to pause (stay stationary), using
    /// `Direction::NoDirection`.
    pub fn pause(&self) -> Order {
        Order {
            pos: self.clone(),
            dir: Direction::NoDirection,
        }
    }
}

impl fmt::Debug for Position {
    // Manually implemented for more compact print out
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pos {{ r: {:?}, c: {:?} }}", self.row, self.col)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone, Copy)]
pub enum Direction {
    North,
    West,
    South,
    East,
    NoDirection,
}

impl Direction {
    /// Reverse the direction
    ///
    /// East becomes West, North becomes South and vice versa.
    pub fn reverse(self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
            NoDirection => NoDirection,
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct Order {
    pub pos: Position,
    pub dir: Direction,
}

impl fmt::Debug for Order {
    // Manually implemented for more compact print out
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Order {{ r:{:?}, c:{:?} -> {:?} }}",
            self.pos.row, self.pos.col, self.dir
        )
    }
}

pub type Orders = Vec<Order>;

/// Helper function (short hand) for a Position.
pub fn pos(row: u16, col: u16) -> Position {
    Position { row, col }
}

impl Position {
    pub fn order(&self, dir: Direction) -> Order {
        Order {
            pos: self.clone(),
            dir,
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
            Order {
                pos: p,
                dir: NoDirection,
            } => p.clone(),
        }
    }

    /// Reverse the order.
    ///
    /// An order East from position (1,1) is reversed
    /// to West from position (1,2).
    pub fn reverse(&self, scope_size: &Position) -> Order {
        let target = self.target_pos(scope_size);
        target.order(self.dir.reverse())
    }
}
