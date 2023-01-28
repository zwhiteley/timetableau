use crate::{Activity, RangedUsize};
#[cfg(feature = "chrono")]
use chrono::prelude::*;
use num_traits::FromPrimitive;
use std::fmt::Debug;

/// The week of a alternating two-week timetable.
///
/// *See the [`crate`] documentation for more information*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Week {
    // Assign the variants integer values such that they can be cast into
    // integers (for mathematical purposes)
    One = 0,
    Two = 1,
}

impl Week {
    /// The number of `Week`s per iteration of the timetable.
    pub const PER_ITERATION: usize = 2;
}

/// An active day in a [`Week`].
///
/// *See the [`crate`] documentation for more information*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveDay {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
}

impl ActiveDay {
    /// The number of `ActiveDay`s per [`Week`].
    pub const PER_WEEK: usize = 5;

    /// The number of days from [`ActiveDay::Monday`].
    ///
    /// | Day                  | Monday | Tuesday | Wednesday | Thursday | Friday |
    /// |----------------------|--------|---------|-----------|----------|--------|
    /// | num_days_from_monday | 0      | 1       | 2         | 3        | 4      |
    pub fn num_days_from_monday(self) -> usize {
        self as usize
    }
}

impl FromPrimitive for ActiveDay {
    fn from_i64(n: i64) -> Option<Self> {
        use ActiveDay::*;

        Some(match n {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            _ => return None,
        })
    }

    fn from_u64(n: u64) -> Option<Self> {
        use ActiveDay::*;

        Some(match n {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            _ => return None,
        })
    }
}

#[cfg(feature = "chrono")]
impl From<ActiveDay> for Weekday {
    fn from(active_day: ActiveDay) -> Self {
        use ActiveDay::*;

        match active_day {
            Monday => Weekday::Mon,
            Tuesday => Weekday::Tue,
            Wednesday => Weekday::Wed,
            Thursday => Weekday::Thu,
            Friday => Weekday::Fri,
        }
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<Weekday> for ActiveDay {
    type Error = ();

    fn try_from(weekday: Weekday) -> Result<Self, Self::Error> {
        use Weekday::*;

        Ok(match weekday {
            Mon => ActiveDay::Monday,
            Tue => ActiveDay::Tuesday,
            Wed => ActiveDay::Wednesday,
            Thu => ActiveDay::Thursday,
            Fri => ActiveDay::Friday,

            // Weekday::Sat and Weekday::Sun are not active days
            _ => return Err(()),
        })
    }
}

/// A period for an [`ActiveDay`].
///
/// *See the [`crate`] documentation for more information*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Period {
    // Assign the variants integer values such that they can be cast into
    // integers (for mathematical purposes)
    /// Tutor time which takes place every morning between `08:25` and `08:50`.
    ///
    /// # Remarks
    ///
    /// This is typically used for the [registration activity](Activity::Registration).
    Tutor = 0,

    /// The first `Period` in a day taking place between `08:50` and
    /// `09:50`.
    First = 1,

    /// The second `Period` in a day taking place between `09:50` and `10:50`.
    Second = 2,

    /// A short recess which takes place between `10:50` and `11:10`.
    Break = 3,

    /// The third `Period` in a day taking place after break between
    /// `11:10` and `12:10`.
    Third = 4,

    /// The fourth `Period` in a day taking place between `12:10` and `13:10`.
    Fourth = 5,

    /// A longer recess, typically for eating lunch, which takes place between `13:10` and
    /// `13:55`.
    Lunch = 6,

    /// The fifth `Period` in a day taking place after lunch between
    /// `13:55` and `14:55`.
    Fifth = 7,
}

impl Period {
    /// The number of `Period`s per [`ActiveDay`].
    pub const PER_DAY: usize = 8;

    /// The number of `Period`s per [`Week`].
    pub const PER_WEEK: usize = Self::PER_DAY * ActiveDay::PER_WEEK;

    /// The number of `Period`s per iteration of the timetable.
    pub const PER_ITERATION: usize = Self::PER_WEEK * Week::PER_ITERATION;

    /// Creates a new `Period` based on the `time` provided -- if the `time`
    /// provided corresponds to a `Period`, that `Period` will be returned,
    /// otherwise [`None`] will be returned.
    ///
    /// *See the [`crate`] documentation for more information*.
    #[cfg(feature = "chrono")]
    pub fn from_time(time: NaiveTime) -> Option<Self> {
        // `time.hour() * 60 + time.minute()` calculates the number of
        // minutes the time is into the day (i.e., the number of minutes
        // since midnight) -- this is done such that we can easily match
        // time ranges (i.e., whether a particular `time` occurs between
        // two other times)
        // Note: all the times in this match statement exclude the upper bound
        match time.hour() * 60 + time.minute() {
            // 08:25 to 08:50
            505..=529 => Some(Period::Tutor),

            // 8:50 to 9:50
            530..=589 => Some(Period::First),

            // 9:50 to 10:50
            590..=649 => Some(Period::Second),

            // 10:50 to 11:10
            650..=669 => Some(Period::Break),

            // 11:10 to 12:10
            670..=729 => Some(Period::Third),

            // 12:10 to 13:10
            730..=789 => Some(Period::Fourth),

            // 13:10 to 13:55
            790..=834 => Some(Period::Lunch),

            // 13:55 to 14:55
            835..=894 => Some(Period::Fifth),

            // No other times are allocated
            _ => None,
        }
    }

    fn with_index(index: usize) -> Option<Self> {
        Some(match index {
            0 => Self::Tutor,
            1 => Self::First,
            2 => Self::Second,
            3 => Self::Break,
            4 => Self::Third,
            5 => Self::Fourth,
            6 => Self::Lunch,
            7 => Self::Fifth,

            _ => return None,
        })
    }
}

/// A specific timeslot on Highfield's two-week alternating timetable.
///
/// *See the [`crate`] documentation for more information*.
///
/// # TimeSlot Indexes
///
/// Each timeslot is assigned a unique *index*[^1] depending on its
/// chronological position within the timetable -- for example, the very first
/// timeslot (`W1MP1`) has an index of `0` and the very last timeslot (`W2FP5`)
/// has an index of `79` (as there are eighty distinct timeslots in Highfield's
/// timetable); every other timeslot has an index between the two.
///
/// The primary purpose of timeslot indexes is to index arrays/lists -- for
/// example, if you represent a timetable as an array of [`TimeSlot::PER_ITERATION] lessons,
///  you can use the index of timeslots as the index for that array (assuming the lessons
/// are chronologically ordered with respect to the start of the iteration).
///
/// [^1]: It should be noted that, as stated in the
///       [crate documentation](crate), timeslots are iteration independent -- this
///       means that a `TimeSlot` created at a later time will only have a greater
///       index than a `TimeSlot` created at an earlier time if it is reflected
///       the the `TimeSlot`s positions relative to the start of their respective
///       iterations (i.e., `I5W1FP5.index()` will be smaller than `I1W2FP5.index()`
///       as [`Week::Two`] occurs after [`Week::One`] when compared iteration
///       independently).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeSlot {
    pub week: Week,
    pub day: ActiveDay,
    pub period: Period,
}

impl TimeSlot {
    /// The number of `TimeSlot`s per [`ActiveDay`].
    ///
    /// # Remarks
    ///
    /// This is the same as [`Period::PER_DAY`] as a [`Period`] is the most granular component of
    /// a `TimeSlot`.
    pub const PER_DAY: usize = Period::PER_DAY;

    /// The number of `TimeSlot`s per [`Week`].
    ///
    /// # Remarks
    ///
    /// This is the same as [`Period::PER_WEEK`] as a [`Period`] is the most granular component of
    /// a `TimeSlot`.
    pub const PER_WEEK: usize = Period::PER_WEEK;

    /// The number of `TimeSlot`s per iteration of the timetable.
    ///
    /// # Remarks
    ///
    /// This is the same as [`Period::PER_ITERATION`] as a [`Period`] is the most granular
    /// component of a `TimeSlot`.
    pub const PER_ITERATION: usize = Period::PER_ITERATION;

    /// Create a `TimeSlot` with an index of `index`.
    ///
    /// *See the [period index](TimeSlot#timeslot-indexes) documentation for
    /// more information*.
    ///
    /// # Remarks
    ///
    /// It is recommended that you use the normal constructor, or the
    /// [`crate::timeslot!`] macro if you want to hardcode a value, as it makes the
    /// code significantly easier to understand.
    pub fn with_index(index: RangedUsize<0, { Self::PER_ITERATION - 1 }>) -> Self {
        // Get the inner value of the RangedU8 -- the reason a RangedU8 is used
        // is to avoid bounds checks (e.g., if a consumer passes an index of
        // `255` to the function).
        let index = index.get();

        Self {
            // `index` = `week_number * PER_WEEK + day_number * PER_DAY + period_number`
            // `index / PER_WEEK` = `week_number`
            week: if index / Self::PER_WEEK == 0 {
                Week::One
            } else {
                Week::Two
            },
            // `index % PER_WEEK` = `day_number * PER_DAY + period_number`
            // `(index % PER_WEEK) / PER DAY` = `day_number`
            day: ActiveDay::from_usize((index % Self::PER_WEEK) / Self::PER_DAY).unwrap(),

            // `index % PER_DAY` = `period_number`
            period: Period::with_index(index % Self::PER_DAY).unwrap(),
        }
    }

    /// Creates a new `TimeSlot` based on the `datetime` -- if the `datetime`
    /// takes place during a timeslot's allocated time, that period will be
    /// returned, if the `datetime` does not take place during any timeslot's
    /// allocated time, [`None`] will be returned instead.
    ///
    /// *See the [`crate`] documentation for more information*.
    ///
    /// # Remarks
    ///
    /// The reason why a `week` parameter is required is because the
    /// `week` cannot be created using the `datetime` alone -- at the time
    /// of writing, there is no known and reliable way to determine the week
    /// based on the date alone.
    #[cfg(feature = "chrono")]
    pub fn from_datetime<Tz>(week: Week, datetime: DateTime<Tz>) -> Option<Self>
    where
        Tz: TimeZone,
    {
        // Convert the `Weekday` to an `ActiveDay`
        // This fails if the `Weekday` is not an active day (i.e., if `Weekday` and `Weekday::Sat`
        // or `Weekday::Sun`)
        let day = datetime.weekday().try_into().ok()?;

        // Retrieve the correct TimeSlot for the `datetime` provided
        // Return `None` if no TimeSlot correspond to the `datetime`
        // provided
        let time_slot = Period::from_time(datetime.time())?;

        Some(Self {
            week,
            day,
            period: time_slot,
        })
    }

    /// Retrieves the `index` of the `TimeSlot`.
    ///
    /// *See the [period index documentation](TimeSlot#timeslot-indexes) for
    /// more information*.
    ///
    /// # Remarks
    ///
    /// The value returned will always be in the range `0..TimeSlot::PER_ITERATION` (and, as a
    /// result, this method can be used to index an array/vector of [`TimeSlot::PER_ITERATION`]
    /// elements).
    pub fn index(self) -> usize {
        (self.week as usize) * Self::PER_WEEK
            + self.day.num_days_from_monday() * Self::PER_DAY
            + self.period as usize
    }
}

/// Creates a [`TimeSlot`] from its `WDP` format.
///
/// *See the [`crate`] documentation for more information*.
///
/// # Examples
///
/// ```
/// # use timetableau::{Week, TimeSlot, timeslot, Period};
/// # use chrono::prelude::*;
/// #
/// # fn main() {
/// // Create the week one thursday second period
/// let timeslot = timeslot!(W1RP2);
///
/// assert_eq!(timeslot.week, Week::One);
/// assert_eq!(timeslot.day, Weekday::Thu);
/// assert_eq!(timeslot.period, Period::Second)
/// # }
/// ```
///
/// # Remarks
///
/// The `WDP` format provided **MUST** be uppercase -- lowercase `WDP`s will
/// fail to match.
#[macro_export]
macro_rules! timeslot {
    (W1MPT) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(0).unwrap())
    };
    (W1MP1) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(1).unwrap())
    };
    (W1MP2) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(2).unwrap())
    };
    (W1MPB) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(3).unwrap())
    };
    (W1MP3) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(4).unwrap())
    };
    (W1MP4) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(5).unwrap())
    };
    (W1MPL) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(6).unwrap())
    };
    (W1MP5) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(7).unwrap())
    };
    (W1TPT) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(8).unwrap())
    };
    (W1TP1) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(9).unwrap())
    };
    (W1TP2) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(10).unwrap())
    };
    (W1TPB) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(11).unwrap())
    };
    (W1TP3) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(12).unwrap())
    };
    (W1TP4) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(13).unwrap())
    };
    (W1TPL) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(14).unwrap())
    };
    (W1TP5) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(15).unwrap())
    };
    (W1WPT) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(16).unwrap())
    };
    (W1WP1) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(17).unwrap())
    };
    (W1WP2) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(18).unwrap())
    };
    (W1WPB) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(19).unwrap())
    };
    (W1WP3) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(20).unwrap())
    };
    (W1WP4) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(21).unwrap())
    };
    (W1WPL) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(22).unwrap())
    };
    (W1WP5) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(23).unwrap())
    };
    (W1RPT) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(24).unwrap())
    };
    (W1RP1) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(25).unwrap())
    };
    (W1RP2) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(26).unwrap())
    };
    (W1RPB) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(27).unwrap())
    };
    (W1RP3) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(28).unwrap())
    };
    (W1RP4) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(29).unwrap())
    };
    (W1RPL) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(30).unwrap())
    };
    (W1RP5) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(31).unwrap())
    };
    (W1FPT) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(32).unwrap())
    };
    (W1FP1) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(33).unwrap())
    };
    (W1FP2) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(34).unwrap())
    };
    (W1FPB) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(35).unwrap())
    };
    (W1FP3) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(36).unwrap())
    };
    (W1FP4) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(37).unwrap())
    };
    (W1FPL) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(38).unwrap())
    };
    (W1FP5) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(39).unwrap())
    };
    (W2MPT) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(40).unwrap())
    };
    (W2MP1) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(41).unwrap())
    };
    (W2MP2) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(42).unwrap())
    };
    (W2MPB) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(43).unwrap())
    };
    (W2MP3) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(44).unwrap())
    };
    (W2MP4) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(45).unwrap())
    };
    (W2MPL) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(46).unwrap())
    };
    (W2MP5) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(47).unwrap())
    };
    (W2TPT) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(48).unwrap())
    };
    (W2TP1) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(49).unwrap())
    };
    (W2TP2) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(50).unwrap())
    };
    (W2TPB) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(51).unwrap())
    };
    (W2TP3) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(52).unwrap())
    };
    (W2TP4) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(53).unwrap())
    };
    (W2TPL) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(54).unwrap())
    };
    (W2TP5) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(55).unwrap())
    };
    (W2WPT) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(56).unwrap())
    };
    (W2WP1) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(57).unwrap())
    };
    (W2WP2) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(58).unwrap())
    };
    (W2WPB) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(59).unwrap())
    };
    (W2WP3) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(60).unwrap())
    };
    (W2WP4) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(61).unwrap())
    };
    (W2WPL) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(62).unwrap())
    };
    (W2WP5) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(63).unwrap())
    };
    (W2RPT) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(64).unwrap())
    };
    (W2RP1) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(65).unwrap())
    };
    (W2RP2) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(66).unwrap())
    };
    (W2RPB) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(67).unwrap())
    };
    (W2RP3) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(68).unwrap())
    };
    (W2RP4) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(69).unwrap())
    };
    (W2RPL) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(70).unwrap())
    };
    (W2RP5) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(71).unwrap())
    };
    (W2FPT) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(72).unwrap())
    };
    (W2FP1) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(73).unwrap())
    };
    (W2FP2) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(74).unwrap())
    };
    (W2FPB) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(75).unwrap())
    };
    (W2FP3) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(76).unwrap())
    };
    (W2FP4) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(77).unwrap())
    };
    (W2FPL) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(78).unwrap())
    };
    (W2FP5) => {
        $crate::TimeSlot::with_index($crate::RangedUsize::new(79).unwrap())
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RangedU8;

    #[cfg(feature = "chrono")]
    #[test]
    fn period_valid() {
        let period = Period::from_time(NaiveTime::from_hms_opt(12, 40, 23).unwrap());

        assert_eq!(period, Some(Period::Fourth))
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn period_boundary() {
        let period_lower = Period::from_time(NaiveTime::from_hms_opt(8, 50, 22).unwrap());

        let period_upper = Period::from_time(NaiveTime::from_hms_opt(14, 54, 45).unwrap());

        assert_eq!(period_lower, Some(Period::First));
        assert_eq!(period_upper, Some(Period::Fifth));
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn period_invalid() {
        let period = Period::from_time(NaiveTime::from_hms_opt(8, 24, 59).unwrap());

        assert_eq!(period, None);
    }

    #[test]
    fn timeslot_index_valid() {
        let timeslot = TimeSlot::with_index(RangedUsize::new(23).unwrap());

        assert_eq!(timeslot.week, Week::One);
        assert_eq!(timeslot.day, ActiveDay::Wednesday);
        assert_eq!(timeslot.period, Period::Fifth);
        assert_eq!(timeslot.index(), 23);
    }

    #[test]
    fn timeslot_index_boundary() {
        let timeslot_lower = TimeSlot::with_index(RangedUsize::new(0).unwrap());

        let timeslot_upper = TimeSlot::with_index(RangedUsize::new(79).unwrap());

        assert_eq!(timeslot_lower.week, Week::One);
        assert_eq!(timeslot_lower.day, ActiveDay::Monday);
        assert_eq!(timeslot_lower.period, Period::Tutor);
        assert_eq!(timeslot_lower.index(), 0);

        assert_eq!(timeslot_upper.week, Week::Two);
        assert_eq!(timeslot_upper.day, ActiveDay::Friday);
        assert_eq!(timeslot_upper.period, Period::Fifth);
        assert_eq!(timeslot_upper.index(), 79);
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn timeslot_time_valid() {
        let timeslot = TimeSlot::from_datetime(
            Week::Two,
            Utc.with_ymd_and_hms(2023, 1, 2, 10, 30, 10).unwrap(),
        );

        assert_eq!(
            timeslot,
            Some(TimeSlot {
                week: Week::Two,
                day: ActiveDay::Monday,
                period: Period::Second
            })
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn timeslot_time_boundary() {
        let timeslot_lower = TimeSlot::from_datetime(
            Week::One,
            Utc.with_ymd_and_hms(2023, 1, 2, 8, 25, 0).unwrap(),
        );

        let timeslot_upper = TimeSlot::from_datetime(
            Week::Two,
            Utc.with_ymd_and_hms(2023, 1, 13, 14, 54, 59).unwrap(),
        );

        assert_eq!(
            timeslot_lower,
            Some(TimeSlot::with_index(RangedUsize::new(0).unwrap()))
        );
        assert_eq!(
            timeslot_upper,
            Some(TimeSlot::with_index(RangedUsize::new(79).unwrap()))
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn timeslot_time_invalid() {
        let timeslot = TimeSlot::from_datetime(
            Week::One,
            Utc.with_ymd_and_hms(2023, 1, 1, 8, 50, 0).unwrap(),
        );

        assert_eq!(timeslot, None);
    }

    #[test]
    fn macro_valid() {
        let timeslot = timeslot!(W2RP3);

        assert_eq!(timeslot.week, Week::Two);
        assert_eq!(timeslot.day, ActiveDay::Thursday);
        assert_eq!(timeslot.period, Period::Third);
    }
}
