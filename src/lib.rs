pub mod agent_actions;
pub mod game_parameters;
pub mod position;
pub mod world_state;

use self::agent_actions::{AgentActions, Direction};
use self::game_parameters::GameParameters;
use self::position::{pos, Position};
use self::world_state::WorldState;

#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
pub struct Score {
    pub per_player: Vec<u64>,
}

pub trait Agent {
    fn prepare(&mut self, params: &GameParameters);
    fn make_turn(&mut self, params: &GameParameters, world: &WorldState) -> AgentActions;
    fn at_end(&mut self, params: &GameParameters, world: &WorldState, score: Score);
}

// TODO Add examples and coumentation, e.g.
// > [dependencies]
// > ants_ai_challenge_api = { git = "https://github.com/..." } 


// TODO use parser combinators in parse methods, like 
// e.g. Nom (https://github.com/Geal/nom)
// or Combine (https://github.com/Marwes/combine)

fn parse_turn_0_lines<I>(lines_iter: &mut I) -> GameParameters
where
    I: Iterator<Item = String>,
{
    let mut result = GameParameters::default();

    for line in lines_iter {
        let mut ws = line.split_whitespace();
        match (ws.next(), ws.next()) {
            (Some("ready"), _) => break,
            (Some(x), Some(y)) => {
                result.put(x, y);
            }
            (x, y) => eprintln!("Unknown tokens when parsing turn 0: {:?} {:?}", x, y),
        }
    }
    result
}

fn parse_turn_x_lines<I>(lines_iter: &mut I) -> WorldState
where
    I: Iterator<Item = String>,
{
    fn parse_row(row: &str) -> u16 {
        row.parse().expect("Row")
    }
    fn parse_col(col: &str) -> u16 {
        col.parse().expect("Col")
    }
    fn parse_owner(owner: &str) -> u8 {
        owner.parse().expect("Owner")
    }

    let mut result = WorldState::default();

    for line in lines_iter {
        let mut ws = line.split_whitespace();
        match (ws.next(), ws.next(), ws.next(), ws.next()) {
            (Some("go"), _, _, _) => break,
            // water
            (Some("w"), Some(r), Some(c), None) => {
                result = result.water(parse_row(r), parse_col(c))
            }
            // food
            (Some("f"), Some(r), Some(c), None) => result = result.food(parse_row(r), parse_col(c)),
            // hill
            (Some("h"), Some(r), Some(c), Some(o)) => {
                result = result.hill(parse_row(r), parse_col(c), parse_owner(o))
            }
            // live ants
            (Some("a"), Some(r), Some(c), Some(o)) => {
                result = result.live_ant(parse_row(r), parse_col(c), parse_owner(o))
            }
            // dead ants
            (Some("d"), Some(r), Some(c), Some(o)) => {
                result = result.dead_ant(parse_row(r), parse_col(c), parse_owner(o))
            }
            // bad input
            (a, b, c, d) => eprintln!(
                "Unknown tokens when parsing turn X: {:?} {:?} {:?} {:?}",
                a, b, c, d
            ),
        }
    }
    result
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

fn serialize_actions(actions: &AgentActions) -> String {
    let mut result = String::from("");

    for (position, direction) in &actions.actions {
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

pub fn run_game(agent: &mut Agent) {
    use std::io::prelude::*;

let std_in = std::io::stdin();
    let mut lines_in = std_in.lock().lines().map(|line| line.unwrap());

    let mut out = |line| print!("{}", line);

    run_game_with_io(agent, &mut lines_in, &mut out);
}

pub fn run_game_with_io<I, O>(agent: &mut Agent, mut lines_iter: I, outln: &mut O)
where
    I: Iterator<Item = String>,
    O: FnMut(String) -> (),
{
    let mut maybe_params: Option<GameParameters> = None;

    while match lines_iter.next().as_ref().map(String::as_ref) {
        Some("") => {
            // empty line
            true
        }
        Some("turn 0") => {
            let params = parse_turn_0_lines(&mut lines_iter);
            agent.prepare(&params);
            maybe_params = Some(params);
            outln(String::from("go\n"));
            true
        }
        Some(x) if x.starts_with("turn") => {
            let params = maybe_params.expect("initialized in turn 0");
            let world = parse_turn_x_lines(&mut lines_iter);
            let actions = agent.make_turn(&params, &world);
            let output = serialize_actions(&actions);
            outln(output);
            outln(String::from("go\n"));
            true
        }
        Some("end") => {
            let params = maybe_params.expect("initialized in turn 0");
            let (world, score) = parse_end_lines(&mut lines_iter);
            agent.at_end(&params, &world, score);

            false
        }
        Some(x) => {
            eprintln!("Unexpected input line <{:?}>. Exiting", x);
            false
        }
        _ => false,
    } {
        // do noting in loop
    }
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

        let actual = parse_turn_0_lines(&mut input.lines().map(|s| String::from(s)));
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
            .food(6, 5)
            .water(7, 6)
            .live_ant(7, 9, 1)
            .live_ant(10, 8, 0)
            .live_ant(10, 9, 0)
            .hill(7, 12, 1);

        let actual = parse_turn_x_lines(&mut input.lines().map(|s| String::from(s)));
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
            .food(6, 5)
            .dead_ant(7, 8, 1)
            .live_ant(9, 8, 0)
            .live_ant(9, 9, 0);

        let expected_score = Score {
            per_player: vec![1, 0],
        };

        let (actual_world_state, actual_score) = parse_end_lines(&mut input.lines().map(|s| String::from(s)));

        assert_eq!(expected_world_state, actual_world_state);
        assert_eq!(expected_score, actual_score);
    }

    #[test]
    fn serialize_actions_success() {
        let mut actions = AgentActions::default();

        actions
            .move_ant(pos(10, 8), Direction::North)
            .move_ant(pos(2, 3), Direction::South)
            .move_ant(pos(4, 5), Direction::East)
            .move_ant(pos(6, 7), Direction::West);

        let expected = indoc!(
            "o 10 8 N
            o 2 3 S
            o 4 5 E
            o 6 7 W
            "
        );

        let actual = serialize_actions(&actions);
        assert_eq!(expected, actual);
    }
}