#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TimeUnit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    #[default]
    Microsecond,
    Nanosecond,
}

impl std::fmt::Debug for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeUnit::Year => write!(f, "Year"),
            TimeUnit::Month => write!(f, "Month"),
            TimeUnit::Day => write!(f, "Day"),
            TimeUnit::Hour => write!(f, "Hour"),
            TimeUnit::Minute => write!(f, "Minute"),
            TimeUnit::Second => write!(f, "Second"),
            TimeUnit::Millisecond => write!(f, "Millisecond"),
            TimeUnit::Microsecond => write!(f, "Microsecond"),
            TimeUnit::Nanosecond => write!(f, "Nanosecond"),
        }
    }
}
