use std::convert::TryFrom;

use async_trait::async_trait;
use rusoto_core::RusotoError;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, DescribeLogGroupsError, DescribeLogGroupsRequest,
    DescribeLogGroupsResponse, LogGroup,
};
use thiserror::Error;

use crate::aws::cwlogs::group::model::{CwlGroup, ParseLogGroupError};

fn cwl_groups_from(log_groups: Option<Vec<LogGroup>>) -> Result<Vec<CwlGroup>, ParseLogGroupError> {
    Ok(log_groups
        .unwrap_or_else(Vec::new)
        .into_iter()
        .map(CwlGroup::try_from)
        .collect::<Result<Vec<_>, _>>()?)
}

#[derive(Debug, Error)]
pub enum CwlGroupCursorError {
    #[error("parse error")]
    Parse {
        #[from]
        source: ParseLogGroupError,
    },

    #[error("rusoto error. cause:{source}")]
    Rusoto {
        #[from]
        source: RusotoError<DescribeLogGroupsError>,
    },
}

#[async_trait]
pub trait CwlGroupCursor {
    async fn next(&mut self) -> Result<Option<Vec<CwlGroup>>, CwlGroupCursorError>;
    async fn refresh(&self) -> Result<Option<Vec<CwlGroup>>, CwlGroupCursorError>;
}

#[derive(Clone)]
pub struct RusotoCwlGroupCursor {
    client: CloudWatchLogsClient,
    request: DescribeLogGroupsRequest,
    next_token: Option<String>,
    has_next: bool,
}

impl RusotoCwlGroupCursor {
    pub fn new(
        client: CloudWatchLogsClient,
        request: DescribeLogGroupsRequest,
    ) -> RusotoCwlGroupCursor {
        RusotoCwlGroupCursor {
            client,
            request,
            next_token: None,
            has_next: true,
        }
    }

    async fn describe_groups(&self) -> Result<DescribeLogGroupsResponse, CwlGroupCursorError> {
        let res = self
            .client
            .describe_log_groups(self.request.clone())
            .await?;

        Ok(res)
    }
}

#[async_trait]
impl CwlGroupCursor for RusotoCwlGroupCursor {
    async fn next(&mut self) -> Result<Option<Vec<CwlGroup>>, CwlGroupCursorError> {
        if self.has_next {
            self.request.next_token = self.next_token.take();

            let res = self.describe_groups().await?;
            self.has_next = res.next_token.is_some();
            self.next_token = res.next_token;

            let groups = cwl_groups_from(res.log_groups)?;
            Ok(Some(groups))
        } else {
            Ok(None)
        }
    }

    async fn refresh(&self) -> Result<Option<Vec<CwlGroup>>, CwlGroupCursorError> {
        let res = self.describe_groups().await?;
        let groups = cwl_groups_from(res.log_groups)?;
        Ok(Some(groups))
    }
}
