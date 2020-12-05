use std::cmp::Ordering;
use std::convert::TryFrom;

use chrono::{DateTime, Utc};
use rusoto_logs::LogStream;
use thiserror::Error;

use crate::aws::cwlogs::mapper::{map_field, map_string_field, map_unix_epoch_millis};
use crate::aws::errors::MissingFieldError;
use crate::aws::Arn;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum ParseLogStreamError {
    #[error("missing field")]
    MissingField {
        #[from]
        source: MissingFieldError,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CwlStream {
    pub arn: Arn,
    pub creation_time: DateTime<Utc>,
    pub stream_name: String,
    pub first_event_time: DateTime<Utc>,
    pub last_event_time: DateTime<Utc>,
    pub last_ingestion_time: DateTime<Utc>,
}

impl PartialOrd for CwlStream {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CwlStream {
    fn cmp(&self, other: &Self) -> Ordering {
        self.arn.cmp(&other.arn)
    }
}

impl TryFrom<LogStream> for CwlStream {
    type Error = ParseLogStreamError;

    fn try_from(stream: LogStream) -> Result<Self, Self::Error> {
        let arn = map_string_field(stream.arn, "arn")?;
        let creation_time = map_unix_epoch_millis(stream.creation_time, "creation_time", Utc)?;
        let stream_name = map_field(stream.log_stream_name, "log_stream_name")?;
        let first_event_time =
            map_unix_epoch_millis(stream.first_event_timestamp, "first_event_timestamp", Utc)?;
        let last_event_time =
            map_unix_epoch_millis(stream.last_event_timestamp, "last_event_timestamp", Utc)?;
        let last_ingestion_time =
            map_unix_epoch_millis(stream.last_ingestion_time, "last_ingestion_time", Utc)?;

        Ok(CwlStream {
            arn,
            creation_time,
            stream_name,
            first_event_time,
            last_event_time,
            last_ingestion_time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    fn log_stream() -> LogStream {
        let s = |s: &str| s.to_string();
        LogStream {
            arn: Some(s("arn:aws:logs:ap-northeast-1:000000000000:log-group:LOG-GROUP:log-stream:LOG-STREAM")),
            creation_time: Some(1604316153000), // 2020-11-02T11:22:33
            log_stream_name: Some(s("LOG-STREAM")),
            first_event_timestamp: Some(1604316153001), // 2020-11-02T11:22:33.001
            last_event_timestamp: Some(1604316153002), // 2020-11-02T11:22:33.002
            last_ingestion_time: Some(1604316153003), // 2020-11-02T11:22:33.003
            ..Default::default()
        }
    }

    #[test]
    fn test_try_from_success() {
        assert_eq!(
            Ok(CwlStream {
                arn: Arn::from("arn:aws:logs:ap-northeast-1:000000000000:log-group:LOG-GROUP:log-stream:LOG-STREAM"),
                creation_time: Utc.ymd(2020, 11, 2).and_hms(11, 22, 33),
                stream_name: String::from("LOG-STREAM"),
                first_event_time: Utc.ymd(2020, 11, 2).and_hms_milli(11, 22, 33, 1),
                last_event_time: Utc.ymd(2020, 11, 2).and_hms_milli(11, 22, 33, 2),
                last_ingestion_time: Utc.ymd(2020, 11, 2).and_hms_milli(11, 22, 33, 3),
            }),
            CwlStream::try_from(log_stream()),
        );
    }

    #[test]
    fn test_try_from_error() {
        fn missing_field_error(field_name: &'static str) -> ParseLogStreamError {
            ParseLogStreamError::MissingField {
                source: MissingFieldError::new(field_name),
            }
        }

        assert_eq!(
            Err(missing_field_error("arn")),
            CwlStream::try_from(LogStream {
                arn: None,
                ..log_stream()
            })
        );
        assert_eq!(
            Err(missing_field_error("creation_time")),
            CwlStream::try_from(LogStream {
                creation_time: None,
                ..log_stream()
            })
        );
        assert_eq!(
            Err(missing_field_error("log_stream_name")),
            CwlStream::try_from(LogStream {
                log_stream_name: None,
                ..log_stream()
            })
        );
        assert_eq!(
            Err(missing_field_error("first_event_timestamp")),
            CwlStream::try_from(LogStream {
                first_event_timestamp: None,
                ..log_stream()
            })
        );
        assert_eq!(
            Err(missing_field_error("last_event_timestamp")),
            CwlStream::try_from(LogStream {
                last_event_timestamp: None,
                ..log_stream()
            })
        );
        assert_eq!(
            Err(missing_field_error("last_ingestion_time")),
            CwlStream::try_from(LogStream {
                last_ingestion_time: None,
                ..log_stream()
            })
        );
    }
}
