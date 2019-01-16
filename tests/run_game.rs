extern crate ants_ai_challenge_api;

use ants_ai_challenge_api::*;
use indoc::indoc;

#[derive(Debug, Default)]
struct TestAgent {
    prep_call_count: u32,
    make_turn_call_count: u32,
    at_end_call_count: u32,
    orders_to_make: Orders,
    expected_game_params: GameParameters,
    expected_world_state: WorldState,
    expected_score: Score,
}

impl Agent for TestAgent {
    fn prepare(&mut self, params: GameParameters) {
        self.prep_call_count += 1;
        assert_eq!(
            self.expected_game_params, params,
            "Expecting left but got right"
        );
    }

    fn make_turn(&mut self, world: WorldState, turn_count: u32) -> Orders {
        self.make_turn_call_count += 1;
        assert_eq!(
            self.make_turn_call_count, turn_count,
            "Turn count, expecting left but go right"
        );
        assert_eq!(
            self.expected_world_state, world,
            "Expecting left but got right"
        );
        self.orders_to_make.clone()
    }
}

#[test]
fn run_game_success() {
    let input = indoc!(
        "turn 0
        loadtime 3000
        turntime 1000
        rows 20
        cols 20
        turns 500
        viewradius2 55
        attackradius2 5
        spawnradius2 1
        player_seed 42
        ready

        turn 1
        f 6 5
        w 7 6
        d 7 9 1
        a 10 8 0
        h 7 12 1
        go

        turn 2
        f 6 5
        w 7 6
        d 7 9 1
        a 10 8 0
        h 7 12 1
        go

        end
        players 2
        score 1 0
        f 6 5
        w 7 6
        d 7 9 1
        a 10 8 0
        h 7 12 1
        go
        "
    );
    let mut output = String::from("");
    let mut add_outputln = |line: String| output.push_str(&line);

    let mut test_agent = TestAgent::default();
    assert_eq!(0, test_agent.prep_call_count, "no prepare calls before use");
    assert_eq!(
        0, test_agent.make_turn_call_count,
        "no make_turn calls before use"
    );
    assert_eq!(
        0, test_agent.at_end_call_count,
        "no at_end calls before use"
    );

    test_agent.expected_game_params = GameParameters {
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

    test_agent.expected_world_state = WorldState::default()
        .food(pos(6, 5))
        .water(pos(7, 6))
        .dead_ant(pos(7, 9), 1)
        .live_ant(pos(10, 8), 0)
        .hill(pos(7, 12), 1);

    test_agent.orders_to_make = vec![(pos(1, 2), North)];

    let (world_at_end, score) = run_game_with_io(
        &mut test_agent,
        input.lines().map(String::from),
        &mut add_outputln,
    );

    assert_eq!(1, test_agent.prep_call_count, "one prepare call after use");
    assert_eq!(
        2, test_agent.make_turn_call_count,
        "two make_turn calls after use"
    );

    assert_eq!(
        test_agent.expected_world_state, world_at_end,
        "WorldState at game end"
    );

    let expected_score = Score {
        per_player: vec![1, 0],
    };
    assert_eq!(expected_score, score, "Score at game end");

    assert_eq!(
        indoc!(
            "go
        o 1 2 N
        go
        o 1 2 N
        go
        "
        ),
        output,
        "Output of agent, expecting left but got right"
    );
}
