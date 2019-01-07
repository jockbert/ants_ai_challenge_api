
use super::{pos, Position};

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
    pub fn food(mut self, row: u16, column: u16) -> Self {
        self.foods.push(pos(row, column));
        self
    }
    /// Add water at given position
    pub fn water(mut self, row: u16, column: u16) -> Self {
        self.waters.push(pos(row, column));
        self
    }

    /// Add live ant at given position for a player
    pub fn live_ant(mut self, row: u16, column: u16, player: u8) -> Self {
        // ensure capacity
        while self.live_ants.len() <= player as usize {
            self.live_ants.push(vec![])
        }

        self.live_ants
            .get_mut(player as usize)
            .unwrap()
            .push(pos(row, column));
        self
    }

    /// Add dead ant at given position for a player
    pub fn dead_ant(mut self, row: u16, column: u16, player: u8) -> Self {
        // ensure capacity
        while self.dead_ants.len() <= player as usize {
            self.dead_ants.push(vec![])
        }

        self.dead_ants
            .get_mut(player as usize)
            .unwrap()
            .push(pos(row, column));
        self
    }

    /// Add hill at given position for a player
    pub fn hill(mut self, row: u16, column: u16, player: u8) -> Self {
        // ensure capacity
        while self.hills.len() <= player as usize {
            self.hills.push(vec![])
        }

        self.hills
            .get_mut(player as usize)
            .unwrap()
            .push(pos(row, column));
        self
    }

    pub fn max_player_count(&self) -> usize {
        std::cmp::max(
            self.live_ants.len(),
            std::cmp::max(self.dead_ants.len(), self.hills.len()),
        )
    }

    pub fn live_ants_for_player(&self, player: u8) -> Vec<Position> {
        self.live_ants
            .get(player as usize)
            .cloned()
            .unwrap_or(vec![])
    }

    pub fn dead_ants_for_player(&self, player: u8) -> Vec<Position> {
        self.dead_ants
            .get(player as usize)
            .cloned()
            .unwrap_or(vec![])
    }

    pub fn hills_for_player(&self, player: u8) -> Vec<Position> {
        self.hills.get(player as usize).cloned().unwrap_or(vec![])
    }
}

#[cfg(test)]
mod tests {
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
            .food(6, 5)
            .water(7, 6)
            .live_ant(7, 9, 1)
            .live_ant(10, 8, 0)
            .live_ant(10, 9, 0)
            .hill(7, 12, 1)
            .dead_ant(17, 19, 3);

        assert_eq!(expected, actual);
    }

    #[test]
    fn per_player_getters() {
        let actual = WorldState::default()
            .food(6, 5)
            .water(7, 6)
            .live_ant(7, 9, 1)
            .live_ant(10, 8, 0)
            .live_ant(10, 9, 0)
            .hill(7, 12, 1)
            .dead_ant(17, 19, 3);

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
