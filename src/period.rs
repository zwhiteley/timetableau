use crate::RangedU8;
use chrono::prelude::*;
use num_traits::FromPrimitive;
use std::fmt::{self, Debug, Formatter};

/// The week of a alternating two-week timetable.
///
/// *See the [`crate`] documentation for more information*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Week {
    // Assign the variants integer values such that they can be cast into
    // integers (for mathematical purposes)
    WeekOne = 0,
    WeekTwo = 1,
}

/// A timeslot for a day.
///
/// *See the [`crate`] documentation for more information*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSlot {
    // Assign the variants integer values such that they can be cast into
    // integers (for mathematical purposes)

    /// The first `TimeSlot` in a day taking place between `08:50` and
    /// `09:50`.
    First = 0,

    /// The second `TimeSlot` in a day taking place between `09:50` and `10:50`.
    Second = 1,

    /// The third `TimeSlot` in a day taking place after break between
    /// `11:10` and `12:10`.
    Third = 2,

    /// The fourth `TimeSlot` in a day taking place between `12:10` and `13:10`.
    Fourth = 3,

    /// The fifth `TimeSlot` in a day taking place after lunch between
    /// `13:55` and `14:55`.
    Fifth = 4,
}

impl TimeSlot {
    /// Creates a new `TimeSlot` based on the `time` provided -- if the `time`
    /// provided corresponds to a `TimeSlot`, that `TimeSlot` will be returned,
    /// otherwise [`None`] will be returned.
    ///
    /// *See the [`crate`] documentation for more information*.
    pub fn from_time(time: NaiveTime) -> Option<Self> {
        // `time.hour() * 60 + time.minute()` calculates the number of
        // minutes the time is into the day (i.e., the number of minutes
        // since midnight) -- this is done such that we can easily match
        // time ranges (i.e., whether a particular `time` occurs between
        // two other times)
        // Note: all the times in this match statement exclude the upper bound
        match time.hour() * 60 + time.minute() {
            // 8:50 to 9:50
            530..=589 => Some(TimeSlot::First),

            // 9:50 to 10:50
            590..=649 => Some(TimeSlot::Second),

            // 11:10 to 12:10
            670..=729 => Some(TimeSlot::Third),

            // 12:10 to 13:10
            730..=789 => Some(TimeSlot::Fourth),

            // 13:55 to 14:55
            835..=894 => Some(TimeSlot::Fifth),

            // No other times are allocated
            _ => None
        }
    }
}

/// A specific period on Highfield's two-week alternating timetable.
///
/// *See the [`crate`] documentation for more information*.
///
/// # Period Indexes
///
/// Each period is assigned a unique *index*[^1] depending on its chronological
/// position within the timetable -- for example, the very first period
/// (`W1MP1`) has an index of `0` and the very last period (`W2FP5`) has an
/// index of `49` (as there are fifty distinct periods in Highfield's
/// timetable).
///
/// The primary purpose of period indexes is to index arrays/lists -- for
/// example, if you represent a timetable as an array of lessons, you can
/// use the index of periods as the index for that array (assuming the lessons
/// are chronologically ordered).
///
/// [^1]: It should be noted that, as stated in the
///       [crate documentation](crate), periods are time independent and only
///       have meaning relative to the timetable -- this means that a period
///       created for a later time will only have a greater index than a period
///       created at an earlier time if that is reflected in the period's
///       positions within the timetable (i.e., `I5W1MP1.index()` <
///       `I1W2FP5.index()`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Period {
    pub week: Week,
    pub day: Weekday,
    pub time_slot: TimeSlot,
}

impl Period {
    /// Create a `Period` with an index of `index`.
    ///
    /// *See the [period index](Period#period-indexes) documentation for more
    /// information*.
    ///
    /// # Remarks
    ///
    /// It is recommended that you use the normal constructor, or the
    /// [`period!`] macro if you want to hardcode a value, as it makes the
    /// code significantly easier to understand.
    pub fn with_index(index: RangedU8<0, 49>) -> Self {
        // Get the inner value of the RangedU8 -- the reason a RangedU8 is used
        // is to avoid bounds checks (e.g., if a consumer passes an index of
        // `255` to the function).
        let index = index.get();

        Self {
            week: if index / 25 == 0 {
                Week::WeekOne
            } else {
                Week::WeekTwo
            },
            day: Weekday::from_u8((index % 25) / 5).unwrap(),
            time_slot: match index % 5 {
                0 => TimeSlot::First,
                1 => TimeSlot::Second,
                2 => TimeSlot::Third,
                3 => TimeSlot::Fourth,

                // it is impossible for (index % 5) to produce a value larger
                // than `4` -- as the match statement must be exhaustive, if
                // (index % 5) is not in the range `0..=3`, it must have a value
                // of `4`
                _ => TimeSlot::Fifth,
            },
        }
    }

    /// Creates a new `Period` based on the `datetime` -- if the `datetime`
    /// takes place during a period's allocated time, that period will be
    /// returned, if the `datetime` does not take place during any period's
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
    pub fn from_datetime<Tz>(week: Week, datetime: DateTime<Tz>) -> Option<Self>
    where
        Tz: TimeZone,
    {
        let day = datetime.weekday();

        // No periods are allocated slots over the weekend
        if day == Weekday::Sat || day == Weekday::Sun {
            return None;
        }

        // Retrieve the correct TimeSlot for the `datetime` provided
        // Return `None` if no TimeSlot correspond to the `datetime`
        // provided
        let time_slot = TimeSlot::from_time(datetime.time())?;

        Some(Self { week, day, time_slot })
    }

    /// Retrieves the `index` of the `Period`.
    ///
    /// *See the [period index documentation](Period#period-indexes) for
    /// more information*.
    pub fn index(self) -> usize {
        (self.week as usize) * 25
            + (self.day.num_days_from_monday() as usize) * 5
            + self.time_slot as usize
    }
}

/// Creates a [`Period`] from its `WDP` format.
///
/// *See the [`crate`] documentation for more information*.
///
/// # Examples
///
/// ```
/// # use timetableau::{Week, TimeSlot, period};
/// # use chrono::prelude::*;
/// #
/// # fn main() {
/// // Create the week one thursday second period
/// let period = period!(W1RP2);
///
/// assert_eq!(period.week, Week::WeekOne);
/// assert_eq!(period.day, Weekday::Thu);
/// assert_eq!(period.time_slot, TimeSlot::Second)
/// # }
/// ```
///
/// # Remarks
///
/// The `WDP` format provided **MUST** be uppercase -- lowercase `WDP`s will
/// fail to match.
#[macro_export]
macro_rules! period {
    // Week One
    (W1MP1) => {
        $crate::Period::with_index($crate::RangedU8::new(0).unwrap())
    };
    (W1MP2) => {
        $crate::Period::with_index($crate::RangedU8::new(1).unwrap())
    };
    (W1MP3) => {
        $crate::Period::with_index($crate::RangedU8::new(2).unwrap())
    };
    (W1MP4) => {
        $crate::Period::with_index($crate::RangedU8::new(3).unwrap())
    };
    (W1MP5) => {
        $crate::Period::with_index($crate::RangedU8::new(4).unwrap())
    };

    (W1TP1) => {
        $crate::Period::with_index($crate::RangedU8::new(5).unwrap())
    };
    (W1TP2) => {
        $crate::Period::with_index($crate::RangedU8::new(6).unwrap())
    };
    (W1TP3) => {
        $crate::Period::with_index($crate::RangedU8::new(7).unwrap())
    };
    (W1TP4) => {
        $crate::Period::with_index($crate::RangedU8::new(8).unwrap())
    };
    (W1TP5) => {
        $crate::Period::with_index($crate::RangedU8::new(9).unwrap())
    };

    (W1WP1) => {
        $crate::Period::with_index($crate::RangedU8::new(10).unwrap())
    };
    (W1WP2) => {
        $crate::Period::with_index($crate::RangedU8::new(11).unwrap())
    };
    (W1WP3) => {
        $crate::Period::with_index($crate::RangedU8::new(12).unwrap())
    };
    (W1WP4) => {
        $crate::Period::with_index($crate::RangedU8::new(13).unwrap())
    };
    (W1WP5) => {
        $crate::Period::with_index($crate::RangedU8::new(14).unwrap())
    };

    (W1RP1) => {
        $crate::Period::with_index($crate::RangedU8::new(15).unwrap())
    };
    (W1RP2) => {
        $crate::Period::with_index($crate::RangedU8::new(16).unwrap())
    };
    (W1RP3) => {
        $crate::Period::with_index($crate::RangedU8::new(17).unwrap())
    };
    (W1RP4) => {
        $crate::Period::with_index($crate::RangedU8::new(18).unwrap())
    };
    (W1RP5) => {
        $crate::Period::with_index($crate::RangedU8::new(19).unwrap())
    };

    (W1FP1) => {
        $crate::Period::with_index($crate::RangedU8::new(20).unwrap())
    };
    (W1FP2) => {
        $crate::Period::with_index($crate::RangedU8::new(21).unwrap())
    };
    (W1FP3) => {
        $crate::Period::with_index($crate::RangedU8::new(22).unwrap())
    };
    (W1FP4) => {
        $crate::Period::with_index($crate::RangedU8::new(23).unwrap())
    };
    (W1FP5) => {
        $crate::Period::with_index($crate::RangedU8::new(24).unwrap())
    };

    // Week Two
    (W2MP1) => {
        $crate::Period::with_index($crate::RangedU8::new(25).unwrap())
    };
    (W2MP2) => {
        $crate::Period::with_index($crate::RangedU8::new(26).unwrap())
    };
    (W2MP3) => {
        $crate::Period::with_index($crate::RangedU8::new(27).unwrap())
    };
    (W2MP4) => {
        $crate::Period::with_index($crate::RangedU8::new(28).unwrap())
    };
    (W2MP5) => {
        $crate::Period::with_index($crate::RangedU8::new(29).unwrap())
    };

    (W2TP1) => {
        $crate::Period::with_index($crate::RangedU8::new(30).unwrap())
    };
    (W2TP2) => {
        $crate::Period::with_index($crate::RangedU8::new(31).unwrap())
    };
    (W2TP3) => {
        $crate::Period::with_index($crate::RangedU8::new(32).unwrap())
    };
    (W2TP4) => {
        $crate::Period::with_index($crate::RangedU8::new(33).unwrap())
    };
    (W2TP5) => {
        $crate::Period::with_index($crate::RangedU8::new(34).unwrap())
    };

    (W2WP1) => {
        $crate::Period::with_index($crate::RangedU8::new(35).unwrap())
    };
    (W2WP2) => {
        $crate::Period::with_index($crate::RangedU8::new(36).unwrap())
    };
    (W2WP3) => {
        $crate::Period::with_index($crate::RangedU8::new(37).unwrap())
    };
    (W2WP4) => {
        $crate::Period::with_index($crate::RangedU8::new(38).unwrap())
    };
    (W2WP5) => {
        $crate::Period::with_index($crate::RangedU8::new(39).unwrap())
    };

    (W2RP1) => {
        $crate::Period::with_index($crate::RangedU8::new(40).unwrap())
    };
    (W2RP2) => {
        $crate::Period::with_index($crate::RangedU8::new(41).unwrap())
    };
    (W2RP3) => {
        $crate::Period::with_index($crate::RangedU8::new(42).unwrap())
    };
    (W2RP4) => {
        $crate::Period::with_index($crate::RangedU8::new(43).unwrap())
    };
    (W2RP5) => {
        $crate::Period::with_index($crate::RangedU8::new(44).unwrap())
    };

    (W2FP1) => {
        $crate::Period::with_index($crate::RangedU8::new(45).unwrap())
    };
    (W2FP2) => {
        $crate::Period::with_index($crate::RangedU8::new(46).unwrap())
    };
    (W2FP3) => {
        $crate::Period::with_index($crate::RangedU8::new(47).unwrap())
    };
    (W2FP4) => {
        $crate::Period::with_index($crate::RangedU8::new(48).unwrap())
    };
    (W2FP5) => {
        $crate::Period::with_index($crate::RangedU8::new(49).unwrap())
    };
}

pub(crate) use period;

#[cfg(test)]
mod tests {
    use crate::RangedU8;
    use super::*;

    #[test]
    fn time_slot_valid() {
        let time_slot = TimeSlot::from_time(
            NaiveTime::from_hms_opt(12, 40, 23).unwrap()
        );

        assert_eq!(time_slot, Some(TimeSlot::Fourth))
    }

    #[test]
    fn time_slot_boundary() {
        let time_slot_lower = TimeSlot::from_time(
            NaiveTime::from_hms_opt(8, 50, 22).unwrap()
        );

        let time_slot_upper = TimeSlot::from_time(
            NaiveTime::from_hms_opt(14, 54, 45).unwrap()
        );

        assert_eq!(time_slot_lower, Some(TimeSlot::First));
        assert_eq!(time_slot_upper, Some(TimeSlot::Fifth));
    }

    #[test]
    fn time_slot_invalid() {
        let time_slot = TimeSlot::from_time(
            NaiveTime::from_hms_opt(8, 49, 59).unwrap()
        );

        assert_eq!(time_slot, None);
    }

    #[test]
    fn period_index_valid() {
        let period = Period::with_index(
            RangedU8::new(23).unwrap()
        );

        assert_eq!(period.week, Week::WeekOne);
        assert_eq!(period.day, Weekday::Fri);
        assert_eq!(period.time_slot, TimeSlot::Fourth);
        assert_eq!(period.index(), 23);
    }

    #[test]
    fn period_index_boundary() {
        let period_lower = Period::with_index(
            RangedU8::new(0).unwrap()
        );

        let period_upper = Period::with_index(
            RangedU8::new(49).unwrap()
        );

        assert_eq!(period_lower.week, Week::WeekOne);
        assert_eq!(period_lower.day, Weekday::Mon);
        assert_eq!(period_lower.time_slot, TimeSlot::First);
        assert_eq!(period_lower.index(), 0);

        assert_eq!(period_upper.week, Week::WeekTwo);
        assert_eq!(period_upper.day, Weekday::Fri);
        assert_eq!(period_upper.time_slot, TimeSlot::Fifth);
        assert_eq!(period_upper.index(), 49);
    }

    #[test]
    fn period_time_valid() {
        let period = Period::from_datetime(
            Week::WeekTwo,
            Utc.with_ymd_and_hms(
                2023, 1, 2,
                10, 30, 10
            ).unwrap()
        );

        assert_eq!(period, Some(Period {
            week: Week::WeekTwo,
            day: Weekday::Mon,
            time_slot: TimeSlot::Second
        }));
    }

    #[test]
    fn period_time_boundary() {
        let period_lower = Period::from_datetime(
            Week::WeekOne,
            Utc.with_ymd_and_hms(
                2023, 1, 2,
                8, 50, 0
            ).unwrap()
        );

        let period_upper = Period::from_datetime(
            Week::WeekTwo,
            Utc.with_ymd_and_hms(
                2023, 1, 13,
                14, 54, 59
            ).unwrap()
        );

        assert_eq!(period_lower, Some(Period::with_index(
            RangedU8::new(0).unwrap()
        )));
        assert_eq!(period_upper, Some(Period::with_index(
            RangedU8::new(49).unwrap()
        )));
    }

    #[test]
    fn period_time_invalid() {
        let period = Period::from_datetime(
            Week::WeekOne,
            Utc.with_ymd_and_hms(
                2023, 1, 1,
                8, 50, 0
            ).unwrap()
        );

        assert_eq!(period, None);
    }

    #[test]
    fn macro_valid() {
        let period = period!(W2RP3);

        assert_eq!(period.week, Week::WeekTwo);
        assert_eq!(period.day, Weekday::Thu);
        assert_eq!(period.time_slot, TimeSlot::Third);
    }
}
