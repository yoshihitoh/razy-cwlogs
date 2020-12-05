use std::cmp::Ordering;
use std::fmt;

use crate::collection::store::Store;
use crate::collection::{AsStr, Length};
use crate::query::Query;
use rusoto_logs::DescribeLogGroupsRequest;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PresetName(String);

impl<T: Into<String>> From<T> for PresetName {
    fn from(s: T) -> Self {
        PresetName(s.into())
    }
}

impl AsStr for PresetName {
    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for PresetName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Preset {
    pub name: PresetName,
    pub group_name_prefix: Option<String>,
}

impl Preset {
    pub fn new(name: PresetName, group_name_prefix: Option<String>) -> Self {
        Preset {
            name,
            group_name_prefix,
        }
    }
}

impl Default for Preset {
    fn default() -> Self {
        Preset {
            name: PresetName::from(String::default()),
            group_name_prefix: None,
        }
    }
}

impl PartialOrd for Preset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Preset {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl Into<DescribeLogGroupsRequest> for Preset {
    fn into(self) -> DescribeLogGroupsRequest {
        DescribeLogGroupsRequest {
            log_group_name_prefix: self.group_name_prefix,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct PresetStore {
    presets: Store<(usize, Preset)>,
    query: Option<Query>,
    label: String,
}

impl PresetStore {
    pub fn insert(&mut self, preset: Preset) {
        self.presets.insert((self.presets.len() + 1, preset));
    }

    pub fn extend(&mut self, presets: impl Iterator<Item = Preset>) {
        let offset = self.presets.len() + 1;
        self.presets
            .extend(presets.enumerate().map(|(i, p)| (i + offset, p)));
    }

    pub fn set_query(&mut self, query: Option<Query>) {
        self.query = query;
        self.label = if let Some(q) = self.query.as_ref() {
            format!("{} (\"{}\")", default_label(), q.word())
        } else {
            default_label().to_string()
        };
    }

    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Preset> {
        let q = self.query.clone();
        self.presets
            .iter()
            .filter(move |(_, p)| preset_matches(p, q.as_ref()))
            .map(|(_, p)| p)
    }
}

impl Default for PresetStore {
    fn default() -> Self {
        PresetStore {
            presets: Store::default(),
            query: None,
            label: default_label().to_string(),
        }
    }
}

impl Length for PresetStore {
    fn len(&self) -> usize {
        self.iter().count()
    }
}

fn default_label() -> &'static str {
    "Presets"
}

fn preset_matches(preset: &Preset, query: Option<&Query>) -> bool {
    if let Some(q) = query {
        preset.name.as_str().contains(q.word())
    } else {
        true
    }
}
