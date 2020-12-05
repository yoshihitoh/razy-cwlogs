use std::convert::TryFrom;

use crate::aws::cwlogs::event::model::{CwlEvent, ParseLogEventError};
use async_trait::async_trait;
use rusoto_core::RusotoError;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, FilterLogEventsError, FilterLogEventsRequest,
    FilterLogEventsResponse, FilteredLogEvent,
};
use thiserror::Error;

fn cwl_events_from(
    log_events: Option<Vec<FilteredLogEvent>>,
) -> Result<Vec<CwlEvent>, ParseLogEventError> {
    Ok(log_events
        .unwrap_or_else(Vec::new)
        .into_iter()
        .map(CwlEvent::try_from)
        .collect::<Result<Vec<_>, _>>()?)
}

#[derive(Debug, Error)]
pub enum CwlEventCursorError {
    #[error("parse error")]
    Parse {
        #[from]
        source: ParseLogEventError,
    },

    #[error("rusoto error")]
    Rusoto {
        #[from]
        source: RusotoError<FilterLogEventsError>,
    },
}

#[async_trait]
pub trait CwlEventCursor {
    async fn next(&mut self) -> Result<Option<Vec<CwlEvent>>, CwlEventCursorError>;
    async fn refresh(&self) -> Result<Option<Vec<CwlEvent>>, CwlEventCursorError>;
}

pub struct RusotoCwlEventCursor {
    client: CloudWatchLogsClient,
    request: FilterLogEventsRequest,
    next_token: Option<String>,
}

impl RusotoCwlEventCursor {
    pub fn new(
        client: CloudWatchLogsClient,
        request: FilterLogEventsRequest,
    ) -> RusotoCwlEventCursor {
        RusotoCwlEventCursor {
            client,
            request,
            next_token: None,
        }
    }

    async fn filter_events(&self) -> Result<FilterLogEventsResponse, CwlEventCursorError> {
        let res = self.client.filter_log_events(self.request.clone()).await?;
        Ok(res)
    }
}

#[async_trait]
impl CwlEventCursor for RusotoCwlEventCursor {
    async fn next(&mut self) -> Result<Option<Vec<CwlEvent>>, CwlEventCursorError> {
        self.request.next_token = self.next_token.take();

        let res = self.filter_events().await?;
        self.next_token = res.next_token;

        let events = cwl_events_from(res.events)?;
        Ok(Some(events))
    }

    async fn refresh(&self) -> Result<Option<Vec<CwlEvent>>, CwlEventCursorError> {
        let res = self.filter_events().await?;
        let events = cwl_events_from(res.events)?;
        Ok(Some(events))
    }
}
