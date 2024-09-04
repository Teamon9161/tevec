use std::fmt::Debug;
use std::hash::Hash;

macro_rules! define_timeunit {
    ($($name: ident),*) => {
        $(
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct $name;

            impl TimeUnitTrait for $name {
                fn unit() -> TimeUnit {
                    TimeUnit::$name
                }
            }
        )*

        /// Represents different units of time.
        ///
        /// This enum includes various time units from years down to nanoseconds,
        /// allowing for flexible time representations and calculations.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum TimeUnit {
            $($name),*
        }
    };
}

/// A trait for types representing time units.
///
/// This trait is implemented by types that represent specific units of time,
/// such as years, months, days, hours, etc. It provides a common interface
/// for these types and ensures they have certain properties and behaviors.

pub trait TimeUnitTrait: Copy + Clone + Debug + PartialEq + Eq + Hash + PartialOrd + Ord {
    /// Returns the corresponding `TimeUnit` enum variant for this time unit.
    ///
    /// # Returns
    ///
    /// A `TimeUnit` enum variant representing the specific time unit.
    fn unit() -> TimeUnit;
}

define_timeunit!(
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond
);

impl Default for TimeUnit {
    #[inline]
    fn default() -> Self {
        TimeUnit::Nanosecond
    }
}
