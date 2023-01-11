pub use location::{FearnhillRoom, HighfieldBlock, HighfieldFloor, HighfieldRoom, Location};
pub use period::{Period, TimeSlot, Week};
pub use ranged::*;

mod ranged;

/// This module contains data structures which describe the locations of rooms
/// within the Highfield school and the Fearnhill school.
mod location;

mod period;
