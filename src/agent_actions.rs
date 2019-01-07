use super::position::Position;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
pub struct AgentActions {
    pub actions: Vec<(Position, Direction)>,
}

impl AgentActions {
    pub fn move_ant(&mut self, pos: Position, dir: Direction) -> &mut Self {
        self.actions.push((pos, dir));
        self
    }
}
