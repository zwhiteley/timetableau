use crate::RangedU8;
use std::fmt::{self, Display, Formatter, Write};

/// A block at the Highfield school.
///
/// *See the [`crate`] documentation for more information*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighfieldBlock {
    Howard,
    Parker,
    Unwin,
}

impl Display for HighfieldBlock {
    // Format the HighfieldBlock (use that block's identifier)
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        use HighfieldBlock::*;

        match self {
            Howard => formatter.write_char('H'),
            Parker => formatter.write_char('P'),
            Unwin => formatter.write_char('U'),
        }
    }
}

/// A floor of a [`HighfieldBlock`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighfieldFloor {
    /// The ground floor of a block.
    Ground,

    /// An upper level of a block.
    ///
    /// # Remarks
    ///
    /// The level must be in the range `1..=9` -- as the [`Self::Ground`] option
    /// is offered, there is no need for a level `0`, and the Highfield school
    /// will never have more than `9` levels.
    Level(RangedU8<1, 9>),
}

impl Display for HighfieldFloor {
    // Format the HighfieldFloor
    // Use 'G' for the ground floor and the floor number for others
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ground => formatter.write_char('G'),
            Self::Level(level) => write!(formatter, "{}", level.get()),
        }
    }
}

/// A room at the Highfield school.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HighfieldRoom {
    /// The block in which the room is located.
    pub block: HighfieldBlock,

    /// The floor on which the room is located.
    pub floor: HighfieldFloor,

    /// The room number.
    ///
    /// # Remarks
    ///
    /// Must be a number in the range `1..100`.
    // The primary reason [`RangedU8`] is used instead of u8 is to allow the
    // field to be made public
    pub number: RangedU8<1, 99>,
}

impl Display for HighfieldRoom {
    // Format the HighfieldRoom such that it prints its room identifier
    //
    // See the crate level documentation for more information
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.block.fmt(formatter)?;
        self.floor.fmt(formatter)?;

        // Format the room number such that it is padded to two digits
        //
        // For example:
        // `1` will formatted as `01`
        // `27` will formatted as `27`
        // `108` is outside the range for the RangedU8, and we therefore do not
        // have to worry about it
        write!(formatter, "{:0>2}", self.number.get())
    }
}

/// A room at the Fearnhill school.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// non_exhaustive is used to allow new fields to be added to the struct at a
// later time without making a breaking change
// See DL#0001 for more information
#[non_exhaustive]
pub struct FearnhillRoom;

// Default will be implemented as the type is non_exhaustive -- the primary
// reason a constructor is not created instead is because the type is
// non_exhaustive (if a constructor was created, a new parameter would have to
// be added for each field added, which would be a breaking change, defeating
// the purpose of having non_exhaustive in the first place!)
// See DL#0001 for more information
impl Default for FearnhillRoom {
    fn default() -> Self {
        FearnhillRoom
    }
}

impl Display for FearnhillRoom {
    // NOTE: Fearnhill must be printed for all default() instances of
    //       FearnhillRoom to avoid a breaking change
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("Fearnhill")
    }
}

/// A location of a room (in which a lesson can take place) in either the
/// Highfield school or the Fearnhill school.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    /// The location of a room at the Highfield school.
    Highfield(HighfieldRoom),

    /// The location of a room at the Fearnhill school.
    Fearnhill(FearnhillRoom),
}

impl Display for Location {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Highfield(room) => room.fmt(formatter),
            Self::Fearnhill(room) => room.fmt(formatter),
        }
    }
}
