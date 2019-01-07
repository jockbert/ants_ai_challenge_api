/// Position on world map. Upper left corner is position (0,0), i.e column 0 and row 0.
#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
pub struct Position {
    /// Row in cartesian map coordinates, comparable to Y-axis value.
    pub row: u16,
    /// Column in cartesian map coordinates, comparable to X-axis value.
    pub col: u16,
}

/// Helper function (short hand) for a Position.
pub fn pos(row: u16, column: u16) -> Position {
    Position {
        row: row,
        col: column,
    }
}