use crate::Location;
use std::fmt::{self, Display, Formatter, Write};

/// A subject/course which a student can undertake.
///
/// # Remarks
///
/// The `name` of the `Subject` must be an ASCII string with a length in
/// the range `1..=16`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subject(String);

impl Subject {
    /// Creates a new `Subject`.
    ///
    /// Returns [`Some(Subject)`](Some) if `name` is an ASCII string with a
    /// length in the range `1..=16`, and [`None`] otherwise.
    pub fn new(name: String) -> Option<Self> {
        // Check the name's validity
        if name.len() < 1 || name.len() > 16 || !name.is_ascii() {
            None
        } else {
            Some(Self(name))
        }
    }

    /// Retrieve the name of the `Subject`.
    pub fn name(&self) -> &String {
        &self.0
    }
}

impl Display for Subject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// The class for a particular [`Lesson`].
///
/// In some cases, students may have multiple teachers for a particular
/// subject (or, a teacher may teach a particular subject to multiple groups
/// of children) -- as a result, each [`Lesson`] comes with class information
/// to help students/teachers to differentiate between lessons of the same
/// subject.
///
/// *See the [`crate`] level documentation for more information*.
///
/// # Remarks
///
/// The class reference must be an ASCII string with a length in the range
/// `1..=32`.
///
/// [`Lesson`]: Activity::Lesson
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class(String);

impl Class {
    /// Create a new `Class`.
    ///
    /// [`Some(Class)`](Some) is returned if `reference` is a valid ASCII
    /// string with a length in the range `1..=32`, otherwise [`None`] is
    /// returned.
    pub fn new(reference: String) -> Option<Self> {
        // Check the reference's validity
        if reference.len() < 1 || reference.len() > 32 || !reference.is_ascii() {
            None
        } else {
            Some(Self(reference))
        }
    }

    /// Retrieves the reference of the `Class`.
    pub fn reference(&self) -> &String {
        &self.0
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// An activity which can occur over one or more [`TimeSlot`](crate::TimeSlot)s.
///
/// *See the [`crate`] documentation for more information*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Activity {
    /// A lesson.
    Lesson {
        /// The subject of the `Lesson`.
        subject: Subject,

        /// The class of the `Lesson`.
        ///
        /// # Remarks
        ///
        /// In most cases, this is the teacher who teaches the `Lesson`.
        class: Class,

        /// The [`Location`] of the `Lesson` (i.e., the room in which the
        /// lesson takes place).
        location: Location,
    },

    /// Where the students are required to register their presence with a
    /// teacher (but aren't expected to complete any work/learning for a
    /// subject).
    Registration,

    /// A break (where the student is not expected to complete any work and is
    /// free to relax/talk/etc).
    Break,

    /// Where the student will be expected to study any subject of their
    /// choosing at school.
    SchoolStudy,

    /// Similar to [`SchoolStudy`], except the student
    /// is free to study at home instead of school.
    ///
    /// # Remarks
    ///
    /// Students are not obligated to go home should this be a timetabled
    /// activity -- they are free to treat it as [`SchoolStudy`] if
    /// they so wish.
    ///
    /// A student cannot (officially[^1]) have `HomeStudy` for the
    /// [`First`] or [`Second`] periods -- they
    /// must be in school.
    ///
    /// [^1]: Due to the ridiculousness of this rule, a large number of
    ///       students break it (especially on days where they have no
    ///       [`Lesson`]s).
    ///
    /// [`First`]: crate::Period::First
    /// [`Second`]: crate::Period::Second
    /// [`Lesson`]: Self::Lesson
    /// [`SchoolStudy`]: Self::SchoolStudy
    HomeStudy,

    /// The student has a miscellaneous `Activity`.
    ///
    /// # Remarks
    ///
    /// This should **NOT** be used for one-off events (e.g., a doctor's
    /// appointment) -- an `Activity` is a regular and recurring event, not
    /// a one-off event.
    ///
    /// *See the [`crate`] documentation for more information*.
    Miscellaneous(String),
}

impl Display for Activity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Activity::*;

        match self {
            Lesson {
                subject,
                class,
                location,
            } => {
                subject.fmt(f)?;
                f.write_char(' ')?;
                class.fmt(f)?;
                f.write_char(' ')?;
                location.fmt(f)
            }
            Registration => f.write_str("Registration"),
            Break => f.write_str("Break"),
            SchoolStudy => f.write_str("Independent Study"),
            HomeStudy => f.write_str("Home Study"),
            Miscellaneous(description) => description.fmt(f),
        }
    }
}
