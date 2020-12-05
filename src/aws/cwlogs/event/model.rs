use std::convert::TryFrom;

use chrono::{DateTime, Utc};
use rusoto_logs::FilteredLogEvent;
use thiserror::Error;

use crate::aws::cwlogs::mapper::{map_field, map_string_field, map_unix_epoch_millis};
use crate::aws::errors::MissingFieldError;
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EventId(String);

impl From<String> for EventId {
    fn from(s: String) -> Self {
        EventId(s)
    }
}

impl From<&str> for EventId {
    fn from(s: &str) -> Self {
        EventId(s.to_string())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum ParseLogEventError {
    #[error("missing field")]
    MissingField {
        #[from]
        source: MissingFieldError,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CwlEvent {
    pub event_id: EventId,
    pub ingestion_time: DateTime<Utc>,
    pub stream_name: String,
    pub message: String,
    pub event_time: DateTime<Utc>,
}

impl PartialOrd for CwlEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CwlEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.event_id.cmp(&other.event_id)
    }
}

impl TryFrom<FilteredLogEvent> for CwlEvent {
    type Error = ParseLogEventError;

    fn try_from(event: FilteredLogEvent) -> Result<Self, Self::Error> {
        let event_id = map_string_field(event.event_id, "event_id")?;
        let ingestion_time = map_unix_epoch_millis(event.ingestion_time, "ingestion_time", Utc)?;
        let stream_name = map_field(event.log_stream_name, "log_stream_name")?;
        let message = map_field(event.message, "message")?;
        let event_time = map_unix_epoch_millis(event.timestamp, "timestamp", Utc)?;
        Ok(CwlEvent {
            event_id,
            ingestion_time,
            stream_name,
            message,
            event_time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    fn log_event() -> FilteredLogEvent {
        let s = |s: &str| s.to_string();
        FilteredLogEvent {
            event_id: Some(s("EVENT-ID")),
            ingestion_time: Some(1604316153000), // 2020-11-02T11:22:33
            log_stream_name: Some(s("STREAM-NAME")),
            message: Some(s("MESSAGE")),
            timestamp: Some(1604316153111), // 2020-11-02T11:22:33.111
        }
    }

    #[test]
    fn test_try_from_success() {
        assert_eq!(
            Ok(CwlEvent {
                event_id: EventId::from("EVENT-ID"),
                ingestion_time: Utc.ymd(2020, 11, 2).and_hms(11, 22, 33),
                stream_name: String::from("STREAM-NAME"),
                message: String::from("MESSAGE"),
                event_time: Utc.ymd(2020, 11, 2).and_hms_milli(11, 22, 33, 111),
            }),
            CwlEvent::try_from(log_event())
        )
    }

    #[test]
    fn test_try_from_error() {
        fn missing_field_error(field_name: &'static str) -> ParseLogEventError {
            ParseLogEventError::MissingField {
                source: MissingFieldError::new(field_name),
            }
        }

        assert_eq!(
            Err(missing_field_error("event_id")),
            CwlEvent::try_from(FilteredLogEvent {
                event_id: None,
                ..log_event()
            })
        );
        assert_eq!(
            Err(missing_field_error("ingestion_time")),
            CwlEvent::try_from(FilteredLogEvent {
                ingestion_time: None,
                ..log_event()
            })
        );
        assert_eq!(
            Err(missing_field_error("log_stream_name")),
            CwlEvent::try_from(FilteredLogEvent {
                log_stream_name: None,
                ..log_event()
            })
        );
        assert_eq!(
            Err(missing_field_error("message")),
            CwlEvent::try_from(FilteredLogEvent {
                message: None,
                ..log_event()
            })
        );
        assert_eq!(
            Err(missing_field_error("timestamp")),
            CwlEvent::try_from(FilteredLogEvent {
                timestamp: None,
                ..log_event()
            })
        );
    }
}
