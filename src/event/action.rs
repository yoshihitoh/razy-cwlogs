use crate::aws::cwlogs::group::CwlGroup;
use crate::aws::profile::ProfileName;
use crate::preset::Preset;

#[derive(Debug, Clone)]
pub enum Action {
    Search(String),
    RequestLogGroups(ProfileName, Option<Preset>),
    ReceiveLogGroups(Vec<CwlGroup>),
    Error(String),
}
