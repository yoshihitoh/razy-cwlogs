use std::cmp::Ordering;
use std::convert::TryFrom;
use std::time::Duration;

use chrono::{DateTime, Local, Utc};
use rusoto_logs::LogGroup;
use thiserror::Error;

use crate::aws::cwlogs::mapper::{
    map_field, map_string_field, map_unix_epoch_millis, DurationDays,
};
use crate::aws::errors::MissingFieldError;
use crate::aws::Arn;
use crate::size::{Size, SizeError};

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum ParseLogGroupError {
    #[error("missing field")]
    MissingField {
        #[from]
        source: MissingFieldError,
    },
    #[error("wrong size")]
    Size {
        #[from]
        source: SizeError,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CwlGroup {
    pub arn: Arn,
    pub creation_time: DateTime<Utc>,
    pub group_name: String,
    pub retention: Option<Duration>,
    pub stored: Size,
}

impl CwlGroup {
    pub fn creation_time_local(&self) -> DateTime<Local> {
        self.creation_time.with_timezone(&Local)
    }
}

impl PartialOrd for CwlGroup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CwlGroup {
    fn cmp(&self, other: &Self) -> Ordering {
        self.arn.cmp(&other.arn)
    }
}

impl TryFrom<LogGroup> for CwlGroup {
    type Error = ParseLogGroupError;

    fn try_from(group: LogGroup) -> Result<Self, Self::Error> {
        let arn = map_string_field(group.arn, "arn")?;
        let creation_time = map_unix_epoch_millis(group.creation_time, "creation_time", Utc)?;
        let group_name = map_field(group.log_group_name, "log_group_name")?;
        let retention = group
            .retention_in_days
            .map(|days| DurationDays::from(days).into());
        let stored = Size::try_from(map_field(group.stored_bytes, "stored_bytes")?)?;

        Ok(CwlGroup {
            arn,
            creation_time,
            group_name,
            retention,
            stored,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    fn log_group() -> LogGroup {
        let s = |s: &str| s.to_string();
        LogGroup {
            arn: Some(s(
                "arn:aws:logs:ap-northeast-1:000000000000:log-group:log_group_name",
            )),
            log_group_name: Some(s("log_group_name")),
            creation_time: Some(1604316153000), // 2020-11-02T11:22:33
            retention_in_days: Some(3),
            stored_bytes: Some(256 * 1024),
            ..Default::default()
        }
    }

    #[test]
    fn test_try_from_success() {
        assert_eq!(
            Ok(CwlGroup {
                arn: Arn::from("arn:aws:logs:ap-northeast-1:000000000000:log-group:log_group_name"),
                creation_time: Utc.ymd(2020, 11, 2).and_hms(11, 22, 33),
                group_name: String::from("log_group_name"),
                retention: Some(Duration::from_secs(3 * 24 * 60 * 60)),
                stored: Size::new(256 * 1024),
            }),
            CwlGroup::try_from(log_group())
        );
    }

    #[test]
    fn test_try_from_error() {
        fn missing_field_error(field_name: &'static str) -> ParseLogGroupError {
            ParseLogGroupError::MissingField {
                source: MissingFieldError::new(field_name),
            }
        }

        assert_eq!(
            Err(missing_field_error("arn")),
            CwlGroup::try_from(LogGroup {
                arn: None,
                ..log_group()
            })
        );
        assert_eq!(
            Err(missing_field_error("creation_time")),
            CwlGroup::try_from(LogGroup {
                creation_time: None,
                ..log_group()
            })
        );
        assert_eq!(
            Err(missing_field_error("log_group_name")),
            CwlGroup::try_from(LogGroup {
                log_group_name: None,
                ..log_group()
            })
        );
        // `retention_in_days` is an optional field.
        assert_eq!(
            Err(missing_field_error("stored_bytes")),
            CwlGroup::try_from(LogGroup {
                stored_bytes: None,
                ..log_group()
            })
        );
    }
}
