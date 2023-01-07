use std::fmt::{self, Display, Formatter};

macro_rules! ranged_types {
    ( $( $( #[$attr: meta] ) * $name: ident($type: ty); )+ ) => {
        $(
            #[doc = concat!("A ranged `", stringify!($type), "`.")]
            ///
            /// A ranged integer is an integer which must take a specific value within a
            /// range of possible values -- in this case, the value must be in the range
            /// `MIN..=MAX`.
            $(#[$attr])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            #[repr(transparent)] /* use the same representation as a normal type */
            pub struct $name<const MIN: $type, const MAX: $type>($type);

            impl<const MIN: $type, const MAX: $type> $name<MIN, MAX> {
                #[doc = concat!("Creates a new `", stringify!($name), "<MIN, MAX>`.")]
                ///
                /// # Returns
                ///
                /// [`Some`] if the `value` provided is within the range
                /// `MIN..=MAX`, or [`None`] if the `value` provided is outside
                /// that range.
                // There are two reasons panic is not used here:
                //  1. Allows new(u8) to be a const function
                //  2. Allows the consumer to have more control (they can use unwrap()
                //     if they want to panic or they can do something else if they don't
                //     want to panic)
                pub const fn new(value: $type) -> Option<Self> {
                    // Ensure the value is within the range `MIN..=MAX`
                    if value >= MIN && value <= MAX {
                        // If the value is within the range, return Some(v)
                        Some(Self(value))
                    } else {
                        // Otherwise, return None
                        None
                    }
                }

                /// Retrieve the inner value of the
                #[doc = concat!(stringify!($name))]
                /// .
                pub const fn get(self) -> $type {
                    self.0
                }
            }

            impl<const MIN: $type, const MAX: $type> From<$name<MIN, MAX>> for $type {
                fn from(value: $name<MIN, MAX>) -> Self {
                    value.get()
                }
            }

            impl<const MIN: $type, const MAX: $type> TryFrom<$type> for $name<MIN, MAX> {
                type Error = ();

                fn try_from(value: $type) -> Result<Self, Self::Error> {
                    Self::new(value).ok_or(())
                }
            }
        )+
    }
}

// Create the ranged types
ranged_types!(
    RangedU8(u8);

    // Hide these as they aren't used -- having them clutter up documentation
    // is unnecessary: the only reason ranged types were created was because
    // they are not currently in the standard library and pulling in an entire
    // crate for them is overkill
    #[doc(hidden)] RangedU16(u16);
    #[doc(hidden)] RangedU32(u32);
    #[doc(hidden)] RangedU64(u64);
    #[doc(hidden)] RangedI8(i8);
    #[doc(hidden)] RangedI16(i16);
    #[doc(hidden)] RangedI32(i32);
    #[doc(hidden)] RangedI64(i64);
);
