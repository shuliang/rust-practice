use slog::Logger;

use super::PlayingCharacter;
use crate::weapon::PlasmaCannon;

pub struct Player {
    name: String,
    logger: Logger,
    weapon: PlasmaCannon,
}

impl Player {
    pub fn new(logger: &Logger, name: &str) -> Self {
        let player_log = logger.new(o!("Player" => format!("{}", name)));
        let weapon_log = logger.new(o!("PlasmaCannon" => "M435"));
        Self {
            name: name.to_string(),
            logger: player_log,
            weapon: PlasmaCannon(weapon_log),
        }
    }
}

impl PlayingCharacter for Player {
    fn shoot(&self) {
        info!(self.logger, "{} shooting with {}", self.name, self.weapon);
        self.weapon.fire();
    }
}
