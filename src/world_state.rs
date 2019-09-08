use super::Position;

#[derive(PartialEq, Eq, Debug, Default)]
pub struct WorldState {
    /// Food positions
    pub foods: Vec<Position>,
    /// Water positions
    pub waters: Vec<Position>,
    /// Live ant positions for each player (zero indexed).
    pub live_ants: Vec<Vec<Position>>,
    /// Dead ant positions for each player (zero indexed).
    pub dead_ants: Vec<Vec<Position>>,
    /// Hill positions for each player (zero indexed).
    pub hills: Vec<Vec<Position>>,
}

impl WorldState {
    /// Add food at given position
    pub fn food(mut self, pos: Position) -> Self {
        self.foods.push(pos);
        self
    }

    /// Add water at given position
    pub fn water(mut self, pos: Position) -> Self {
        self.waters.push(pos);
        self
    }

    /// Add live ant at given position for a player
    pub fn live_ant(mut self, pos: Position, player: u8) -> Self {
        ensure_capacity(&mut self.live_ants, player);
        self.live_ants[player as usize].push(pos);
        self
    }

    /// Add dead ant at given position for a player
    pub fn dead_ant(mut self, pos: Position, player: u8) -> Self {
        ensure_capacity(&mut self.dead_ants, player);
        self.dead_ants[player as usize].push(pos);
        self
    }

    /// Add hill at given position for a player
    pub fn hill(mut self, pos: Position, player: u8) -> Self {
        ensure_capacity(&mut self.hills, player);
        self.hills[player as usize].push(pos);
        self
    }

    pub fn max_player_count(&self) -> usize {
        std::cmp::max(
            self.live_ants.len(),
            std::cmp::max(self.dead_ants.len(), self.hills.len()),
        )
    }

    pub fn live_ants_for_player(&self, player: u8) -> Vec<Position> {
        get_or_empty(&self.live_ants, player)
    }

    pub fn dead_ants_for_player(&self, player: u8) -> Vec<Position> {
        get_or_empty(&self.dead_ants, player)
    }

    pub fn hills_for_player(&self, player: u8) -> Vec<Position> {
        get_or_empty(&self.hills, player)
    }
}

fn ensure_capacity(vec: &mut Vec<Vec<Position>>, capacity: u8) {
    while vec.len() <= capacity as usize {
        vec.push(vec![])
    }
}

fn get_or_empty(vec: &[Vec<Position>], index: u8) -> Vec<Position> {
    vec.get(index as usize).cloned().unwrap_or_else(|| vec![])
}

#[cfg(test)]
mod tests {
    use super::super::pos;
    use super::*;

    #[test]
    fn use_builder_success() {
        let expected = WorldState {
            foods: vec![pos(6, 5)],
            waters: vec![pos(7, 6)],
            live_ants: vec![vec![pos(10, 8), pos(10, 9)], vec![pos(7, 9)]],
            dead_ants: vec![vec![], vec![], vec![], vec![pos(17, 19)]],
            hills: vec![vec![], vec![pos(7, 12)]],
        };

        let actual = WorldState::default()
            .food(pos(6, 5))
            .water(pos(7, 6))
            .live_ant(pos(7, 9), 1)
            .live_ant(pos(10, 8), 0)
            .live_ant(pos(10, 9), 0)
            .hill(pos(7, 12), 1)
            .dead_ant(pos(17, 19), 3);

        assert_eq!(expected, actual);
    }

    #[test]
    fn per_player_getters() {
        let actual = WorldState::default()
            .food(pos(6, 5))
            .water(pos(7, 6))
            .live_ant(pos(7, 9), 1)
            .live_ant(pos(10, 8), 0)
            .live_ant(pos(10, 9), 0)
            .hill(pos(7, 12), 1)
            .dead_ant(pos(17, 19), 3);

        // Exists player indexes 0-3 for dead ants
        assert_eq!(4, actual.max_player_count());

        assert_eq!(actual.live_ants_for_player(0), vec![pos(10, 8), pos(10, 9)]);
        assert_eq!(actual.live_ants_for_player(1), vec![pos(7, 9)]);
        assert_eq!(actual.live_ants_for_player(2), vec![]);
        assert_eq!(actual.live_ants_for_player(3), vec![]);
        assert_eq!(actual.live_ants_for_player(4), vec![]);
        assert_eq!(actual.live_ants_for_player(5), vec![]);
        assert_eq!(actual.live_ants_for_player(255), vec![]);

        assert_eq!(actual.dead_ants_for_player(0), vec![]);
        assert_eq!(actual.dead_ants_for_player(1), vec![]);
        assert_eq!(actual.dead_ants_for_player(2), vec![]);
        assert_eq!(actual.dead_ants_for_player(3), vec![pos(17, 19)]);

        assert_eq!(actual.hills_for_player(0), vec![]);
        assert_eq!(actual.hills_for_player(1), vec![pos(7, 12)]);
        assert_eq!(actual.hills_for_player(2), vec![]);
        assert_eq!(actual.hills_for_player(3), vec![]);
    }
}
