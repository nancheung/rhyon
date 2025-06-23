use chrono::{DateTime, FixedOffset, Utc};

pub trait Converter<T> {
    fn convert(&self) -> T;
}

impl Converter<DateTime<Utc>> for DateTime<FixedOffset> {
    fn convert(&self) -> DateTime<Utc> {
        self.with_timezone(&Utc)
    }
}

impl Converter<DateTime<FixedOffset>> for DateTime<Utc> {
    fn convert(&self) -> DateTime<FixedOffset> {
        self.with_timezone(&FixedOffset::east_opt(0).unwrap())
    }
}
