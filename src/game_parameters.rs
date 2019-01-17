#[derive(PartialEq, Eq, Debug, Default, Hash)]
pub struct GameParameters {
    // in milliseconds, time given for bot to start up after it is given "ready" (see below)
    pub loadtime_ms: i64,
    // in milliseconds, time given to the bot each turn
    pub turntime_ms: i64,
    // number of rows in the map
    pub rows: i64,
    // number of columns in the map
    pub cols: i64,
    // maximum number of turns in the game
    pub turns: i64,
    // view radius squared
    pub viewradius2: i64,
    // battle radius squared
    pub attackradius2: i64,
    // food gathering radius squared (name is an unfortunate historical artifact)
    pub spawnradius2: i64,
    // seed for random number generator, useful for reproducing games
    pub player_seed: i64,
}
