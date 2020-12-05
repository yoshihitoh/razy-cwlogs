use std::convert::TryFrom;

use async_trait::async_trait;
use rusoto_core::RusotoError;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, DescribeLogStreamsError, DescribeLogStreamsRequest,
    DescribeLogStreamsResponse, LogStream,
};
use thiserror::Error;

use crate::aws::cwlogs::stream::model::{CwlStream, ParseLogStreamError};

fn cwl_streams_from(
    log_streams: Option<Vec<LogStream>>,
) -> Result<Vec<CwlStream>, ParseLogStreamError> {
    Ok(log_streams
        .unwrap_or_else(Vec::new)
        .into_iter()
        .map(CwlStream::try_from)
        .collect::<Result<Vec<_>, _>>()?)
}

#[derive(Debug, Error)]
pub enum CwlStreamCursorError {
    #[error("parse error")]
    Parse {
        #[from]
        source: ParseLogStreamError,
    },

    #[error("rusoto error")]
    Rusoto {
        #[from]
        source: RusotoError<DescribeLogStreamsError>,
    },
}

#[async_trait]
pub trait CwlStreamCursor {
    async fn next(&mut self) -> Result<Option<Vec<CwlStream>>, CwlStreamCursorError>;
    async fn refresh(&self) -> Result<Option<Vec<CwlStream>>, CwlStreamCursorError>;
}

pub struct RusotoCwlStreamCursor {
    client: CloudWatchLogsClient,
    request: DescribeLogStreamsRequest,
    next_token: Option<String>,
    has_next: bool,
}

impl RusotoCwlStreamCursor {
    pub fn new(
        client: CloudWatchLogsClient,
        request: DescribeLogStreamsRequest,
    ) -> RusotoCwlStreamCursor {
        RusotoCwlStreamCursor {
            client,
            request,
            next_token: None,
            has_next: true,
        }
    }

    async fn describe_streams(&self) -> Result<DescribeLogStreamsResponse, CwlStreamCursorError> {
        let res = self
            .client
            .describe_log_streams(self.request.clone())
            .await?;

        Ok(res)
    }
}

#[async_trait]
impl CwlStreamCursor for RusotoCwlStreamCursor {
    async fn next(&mut self) -> Result<Option<Vec<CwlStream>>, CwlStreamCursorError> {
        if self.has_next {
            self.request.next_token = self.next_token.take();

            let res = self.describe_streams().await?;
            self.has_next = res.next_token.is_some();
            self.next_token = res.next_token;

            let streams = cwl_streams_from(res.log_streams)?;
            Ok(Some(streams))
        } else {
            Ok(None)
        }
    }

    async fn refresh(&self) -> Result<Option<Vec<CwlStream>>, CwlStreamCursorError> {
        let res = self.describe_streams().await?;
        let streams = cwl_streams_from(res.log_streams)?;
        Ok(Some(streams))
    }
}
