use slog::Logger;
use std::fmt;

#[derive(Debug)]
pub struct PlasmaCannon(pub Logger);

impl PlasmaCannon {
    pub fn fire(&self) {
        info!(self.0, "Pew Pew !!");
    }
}

impl fmt::Display for PlasmaCannon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(PlasmaCannon))
    }
}

#[derive(Debug)]
pub struct RailGun(pub Logger);

impl RailGun {
    pub fn fire(&self) {
        info!(self.0, "Swoosh !!");
    }
}

impl fmt::Display for RailGun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(RailGun))
    }
}
