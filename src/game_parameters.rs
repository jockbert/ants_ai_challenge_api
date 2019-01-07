
#[derive(PartialEq, Eq, Debug, Default, Hash, Clone, Copy)]
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

impl GameParameters {
    pub fn put(&mut self, key: &str, val: &str) -> &mut Self {
        match val.parse::<i64>() {
            Ok(n) => match key {
                "loadtime" => self.loadtime_ms = n,
                "turntime" => self.turntime_ms = n,
                "rows" => self.rows = n,
                "cols" => self.cols = n,
                "turns" => self.turns = n,
                "viewradius2" => self.viewradius2 = n,
                "attackradius2" => self.attackradius2 = n,
                "spawnradius2" => self.spawnradius2 = n,
                "player_seed" => self.player_seed = n,
                _ => eprintln!(
                    "Unknown game parameter key '{}', with value '{}'.",
                    key, val
                ),
            },
            Err(_) => eprintln!(
                "For game parameter key '{}', unable to parse value '{}' as integer.",
                key, val
            ),
        }
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_success() {
        assert_eq!(33, GameParameters::default().put("rows", "33").rows);
    }

    #[test]
    fn put_ignore_bad_key() {
        assert_eq!(0, GameParameters::default().put("bad key", "33").rows);
    }

    #[test]
    fn put_ignore_bad_value() {
        assert_eq!(0, GameParameters::default().put("rows", "ee").rows);
    }
}
