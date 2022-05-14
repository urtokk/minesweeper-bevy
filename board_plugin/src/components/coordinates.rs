use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};
use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Insectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

// We want to be able to make coordinates sums
impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, other: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// We want to be able to subtract coordinates
impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, other: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
