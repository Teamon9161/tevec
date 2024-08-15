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

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum TimeUnit {
            $($name),*
        }
    };
}

pub trait TimeUnitTrait: Copy + Clone + Debug + PartialEq + Eq + Hash + PartialOrd + Ord {
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
