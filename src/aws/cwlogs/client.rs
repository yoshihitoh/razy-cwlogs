use rusoto_core::credential::{CredentialsError, ProfileProvider};
use rusoto_core::{HttpClient, Region};
use rusoto_logs::CloudWatchLogsClient;
use thiserror::Error;

use crate::aws::profile::ProfileName;
use crate::collection::AsStr;
use rusoto_core::request::TlsError;

#[derive(Debug, Error)]
pub enum ClientFactoryError {
    #[error("TLS error")]
    TlsError(#[from] TlsError),

    #[error("credentials error")]
    Credentials(#[from] CredentialsError),
}

pub struct ClientFactory;

impl ClientFactory {
    pub fn new_client(
        &self,
        profile_name: ProfileName,
    ) -> Result<CloudWatchLogsClient, ClientFactoryError> {
        let dispatcher = HttpClient::new()?;
        let provider = ProfileProvider::with_default_credentials(profile_name.as_str())?;
        // FIXME: get region as an argument.
        let region = Region::default();
        let client = CloudWatchLogsClient::new_with(dispatcher, provider, region);
        Ok(client)
    }
}
