pub mod game_parameters;
pub mod position;
pub mod world_state;

pub use self::game_parameters::GameParameters;
pub use self::position::{pos, Position};
pub use self::world_state::WorldState;
pub use self::Direction::*;

#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
pub struct Score {
    pub per_player: Vec<u64>,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

pub type Order = (Position, Direction);
pub type Orders = Vec<Order>;

pub trait Agent {
    fn prepare(&mut self, params: GameParameters);
    fn make_turn(&mut self, world: WorldState, turn_count: u32) -> Orders;
}

// TODO Add examples and documentation, e.g.
// > [dependencies]
// > ants_ai_challenge_api = { git = "https://github.com/..." }

// TODO use parser combinators in parse methods, like
// e.g. Nom (https://github.com/Geal/nom)
// or Combine (https://github.com/Marwes/combine)

fn parse_turn_0_lines<I>(lines_iter: &mut I) -> GameParameters
where
    I: Iterator<Item = String>,
{
    let seed = GameParameters::default();
    lines_iter
        .take_while(|line| line.trim() != "ready")
        .fold(seed, |mut params, line| {
            let mut tokens = line.split_whitespace();
            let name = tokens.next().expect("Game parameter name");
            let value: i64 = tokens
                .next()
                .expect("Game parameter value")
                .parse()
                .expect("Value should be valud integer");

            match name {
                "loadtime" => params.loadtime_ms = value,
                "turntime" => params.turntime_ms = value,
                "rows" => params.rows = value,
                "cols" => params.cols = value,
                "turns" => params.turns = value,
                "viewradius2" => params.viewradius2 = value,
                "attackradius2" => params.attackradius2 = value,
                "spawnradius2" => params.spawnradius2 = value,
                "player_seed" => params.player_seed = value,
                _ => panic!("Unknown game parameter '{}', with value '{}'.", name, value),
            };
            params
        })
}

fn parse_turn_x_lines<I>(lines_iter: &mut I) -> WorldState
where
    I: Iterator<Item = String>,
{
    fn parse_row(row: &str) -> u16 {
        row.parse().expect("Parsing position row")
    }
    fn parse_col(col: &str) -> u16 {
        col.parse().expect("Parsing position column")
    }
    fn parse_owner(owner: &str) -> u8 {
        owner.parse().expect("Parsing owner")
    }

    lines_iter
        .take_while(|line| line.trim() != "go")
        .fold(WorldState::default(), |world, line| {
            let mut tokens = line.split_whitespace();
            let item = tokens.next();
            let row = tokens.next().map(parse_row);
            let col = tokens.next().map(parse_col);
            let owner = tokens.next().map(parse_owner);

            let pos = match (row, col) {
                (Some(r), Some(c)) => pos(r, c),
                (r, c) => panic!("Bad position when parsing turn X: {:?} {:?}", r, c),
            };

            match (item, owner) {
                (Some("w"), None) => world.water(pos),
                (Some("f"), None) => world.food(pos),
                (Some("h"), Some(o)) => world.hill(pos, o),
                (Some("a"), Some(o)) => world.live_ant(pos, o),
                (Some("d"), Some(o)) => world.dead_ant(pos, o),
                (i, o) => panic!("Bad item {:?} or owner {:?} when parsing turn X", i, o),
            }
        })
}

fn parse_end_lines<I>(lines_iter: &mut I) -> (WorldState, Score)
where
    I: Iterator<Item = String>,
{
    let mut score = Score::default();

    // players line
    let players_line = lines_iter.next().expect("Players line");
    let mut player_tokens = players_line.split_whitespace();
    let players: u8 = match (player_tokens.next(), player_tokens.next()) {
        (Some("players"), Some(p)) => p.parse().expect("Number of players"),
        (a, b) => panic!("Expected 'players <N>', got {:?} {:?}", a, b),
    };

    // Score line
    let score_line = lines_iter.next().expect("Score line");
    let mut score_tokens = score_line.split_whitespace();
    match score_tokens.next() {
        Some("score") => (),
        a => panic!("Keyword 'score' should be first on score line, got {:?}", a),
    };
    for player_score in score_tokens {
        score
            .per_player
            .push(player_score.parse().expect("Parsable player score"))
    }

    // parse score safety check
    if players as usize != score.per_player.len() {
        panic!(
            "expected {} players, but only got {} scores in {:?}",
            players,
            score.per_player.len(),
            score
        )
    }

    // world state lines
    let world_state = parse_turn_x_lines(lines_iter);

    (world_state, score)
}

fn serialize_orders(orders: &[Order]) -> String {
    let mut result = String::from("");

    for (position, direction) in orders {
        result.push_str(&format!("o {} {} ", position.row, position.col));
        result.push(match direction {
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::West => 'W',
            Direction::East => 'E',
        });
        result.push('\n');
    }

    result
}

pub fn run_game(agent: &mut Agent) -> (WorldState, Score) {
    use std::io::prelude::*;

    let std_in = std::io::stdin();
    let mut lines_in = std_in.lock().lines().map(|line| line.unwrap());
    let mut out = |line| print!("{}", line);
    run_game_with_io(agent, &mut lines_in, &mut out)
}

pub fn run_game_with_io<I, O>(
    agent: &mut Agent,
    mut lines_iter: I,
    outln: &mut O,
) -> (WorldState, Score)
where
    I: Iterator<Item = String>,
    O: FnMut(String) -> (),
{
    let mut turn_count: u32 = 0;
    loop {
        match lines_iter.next().as_ref().map(String::as_ref) {
            Some("") => (), /* empty line  */
            Some("turn 0") => {
                let params = parse_turn_0_lines(&mut lines_iter);
                agent.prepare(params);
                outln(String::from("go\n"));
            }
            Some(x) if x.starts_with("turn") => {
                turn_count += 1;
                let world = parse_turn_x_lines(&mut lines_iter);
                let orders = agent.make_turn(world, turn_count);
                let output = serialize_orders(&orders);
                outln(output);
                outln("go\n".to_string());
            }
            Some("end") => break,
            Some(x) => panic!("Unexpected input line <{:?}>", x),
            None => panic!("Unexpected end of input lines"),
        }
    }

    parse_end_lines(&mut lines_iter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_turn_0_success() {
        let input = indoc!(
            "loadtime 3000
            turntime 1000
            rows 20
            cols 20
            turns 500
            viewradius2 55
            attackradius2 5
            spawnradius2 1
            player_seed 42
            ready"
        );
        let expected = GameParameters {
            loadtime_ms: 3000,
            turntime_ms: 1000,
            rows: 20,
            cols: 20,
            turns: 500,
            viewradius2: 55,
            attackradius2: 5,
            spawnradius2: 1,
            player_seed: 42,
        };

        let actual = parse_turn_0_lines(&mut input.lines().map(String::from));
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_turn_1_success() {
        let input = indoc!(
            "f 6 5
            w 7 6
            a 7 9 1
            a 10 8 0
            a 10 9 0
            h 7 12 1
            go"
        );

        let expected = WorldState::default()
            .food(pos(6, 5))
            .water(pos(7, 6))
            .live_ant(pos(7, 9), 1)
            .live_ant(pos(10, 8), 0)
            .live_ant(pos(10, 9), 0)
            .hill(pos(7, 12), 1);

        let actual = parse_turn_x_lines(&mut input.lines().map(String::from));
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_end_success() {
        let input = indoc!(
            "players 2
            score 1 0
            f 6 5
            d 7 8 1
            a 9 8 0
            a 9 9 0
            go"
        );

        let expected_world_state = WorldState::default()
            .food(pos(6, 5))
            .dead_ant(pos(7, 8), 1)
            .live_ant(pos(9, 8), 0)
            .live_ant(pos(9, 9), 0);

        let expected_score = Score {
            per_player: vec![1, 0],
        };

        let (actual_world_state, actual_score) =
            parse_end_lines(&mut input.lines().map(String::from));

        assert_eq!(expected_world_state, actual_world_state);
        assert_eq!(expected_score, actual_score);
    }

    #[test]
    fn serialize_orders_success() {
        let mut orders: Orders = vec![];

        orders.push((pos(10, 8), Direction::North));
        orders.push((pos(2, 3), Direction::South));
        orders.push((pos(4, 5), Direction::East));
        orders.push((pos(6, 7), Direction::West));

        let expected = indoc!(
            "o 10 8 N
            o 2 3 S
            o 4 5 E
            o 6 7 W
            "
        );

        let actual = serialize_orders(&orders);
        assert_eq!(expected, actual);
    }
}
