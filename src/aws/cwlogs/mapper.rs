use std::time::Duration;

use chrono::{DateTime, TimeZone};

use crate::aws::errors::MissingFieldError;

pub fn map_field<T>(
    maybe_field: Option<T>,
    field_name: &'static str,
) -> Result<T, MissingFieldError> {
    if let Some(f) = maybe_field {
        Ok(f)
    } else {
        Err(MissingFieldError::new(field_name))
    }
}

pub fn map_string_field<T: From<String>>(
    maybe_field: Option<String>,
    field_name: &'static str,
) -> Result<T, MissingFieldError> {
    let s = map_field(maybe_field, field_name)?;
    Ok(T::from(s))
}

pub fn map_unix_epoch_millis<Tz: TimeZone>(
    maybe_field: Option<i64>,
    field_name: &'static str,
    tz: Tz,
) -> Result<DateTime<Tz>, MissingFieldError> {
    let ms = map_field(maybe_field, field_name)?;
    Ok(tz.timestamp_millis(ms))
}

#[derive(Debug, Copy, Clone)]
pub struct DurationDays(i64);

impl DurationDays {
    pub fn as_duration(&self) -> Duration {
        Duration::from_secs(self.0 as u64 * 24 * 60 * 60)
    }
}

impl From<i64> for DurationDays {
    fn from(days: i64) -> Self {
        DurationDays(days)
    }
}

impl Into<Duration> for DurationDays {
    fn into(self) -> Duration {
        self.as_duration()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone, Utc};

    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct Name(String);

    impl From<String> for Name {
        fn from(s: String) -> Self {
            Name(s)
        }
    }

    #[derive(Default)]
    struct Record {
        dt: Option<DateTime<Utc>>,
        name: Option<String>,
        time_in_millis: Option<i64>,
    }

    #[test]
    fn test_map_field() {
        let record = Record {
            dt: Some(Utc.ymd(2020, 11, 2).and_hms(11, 22, 33)),
            ..Default::default()
        };
        assert_eq!(
            Ok(Utc.ymd(2020, 11, 2).and_hms(11, 22, 33)),
            map_field(record.dt, "dt")
        );

        assert_eq!(
            Err(MissingFieldError::new("dt")),
            map_field(Record::default().dt, "dt")
        );
    }

    #[test]
    fn test_map_string_field() {
        let record = Record {
            name: Some("NAME".to_string()),
            ..Default::default()
        };
        assert_eq!(
            Ok(Name::from("NAME".to_string())),
            map_string_field(record.name, "name")
        );

        assert_eq!(
            Err(MissingFieldError::new("name")),
            map_string_field::<Name>(Record::default().name, "name")
        );
    }

    #[test]
    fn test_map_unix_epoch_millis() {
        let record = Record {
            time_in_millis: Some(1604316153000),
            ..Default::default()
        };
        assert_eq!(
            Ok(Utc.ymd(2020, 11, 2).and_hms(11, 22, 33)),
            map_unix_epoch_millis(record.time_in_millis, "time_in_millis", Utc),
        );

        assert_eq!(
            Err(MissingFieldError::new("time_in_millis")),
            map_unix_epoch_millis(Record::default().time_in_millis, "time_in_millis", Utc),
        );
    }

    #[test]
    fn test_duration_days() {
        assert_eq!(
            Duration::from_secs(86_400), // 1 * 24 * 60 * 60
            DurationDays::from(1).as_duration(),
        );
    }
}
