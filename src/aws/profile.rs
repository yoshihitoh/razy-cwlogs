use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use dirs_next::home_dir;
use thiserror::Error;

use crate::collection::store::Store;
use crate::collection::{AsStr, Length};
use crate::query::Query;

#[derive(Debug, Error)]
pub enum ProfileStoreError {
    #[error("io error")]
    Io(#[from] io::Error),

    #[error("no home directory found")]
    NoHomeDirectory,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ProfileName(String);

impl AsStr for ProfileName {
    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// FIXME: Replace to Display macro  from derive_more
impl fmt::Display for ProfileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T: Into<String>> From<T> for ProfileName {
    fn from(t: T) -> Self {
        ProfileName(t.into())
    }
}

#[derive(Debug)]
pub struct ProfileStore {
    profile_names: Store<(usize, ProfileName)>,
    query: Option<Query>,
    label: String,
}

impl ProfileStore {
    pub fn from_shared_file() -> Result<ProfileStore, ProfileStoreError> {
        Self::from_file(default_config_path().ok_or(ProfileStoreError::NoHomeDirectory)?)
    }

    pub fn from_file(path: PathBuf) -> Result<ProfileStore, ProfileStoreError> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let profile_names = reader.lines().filter_map(|r| match r {
            Ok(s) => {
                let trimmed = s.trim();
                if trimmed.starts_with("[") && trimmed.ends_with("]") {
                    let trimmed = trimmed[1..(trimmed.len() - 1)].trim();
                    let name = if trimmed.starts_with("profile ") {
                        &trimmed[("profile ".len())..]
                    } else {
                        trimmed
                    };

                    Some(Ok(ProfileName::from(name.trim())))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        });

        let mut store = ProfileStore::default();
        for p in profile_names {
            store.insert(p?);
        }
        Ok(store)
    }

    pub fn insert(&mut self, profile: ProfileName) {
        self.profile_names
            .insert((self.profile_names.len() + 1, profile));
    }

    pub fn extend(&mut self, profiles: impl Iterator<Item = ProfileName>) {
        let offset = self.profile_names.len() + 1;
        self.profile_names
            .extend(profiles.enumerate().map(|(i, p)| (i + offset, p)));
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

    pub fn iter(&self) -> impl Iterator<Item = &ProfileName> {
        let q = self.query.clone();
        self.profile_names
            .iter()
            .filter(move |(_, p)| profile_matches(p, q.as_ref()))
            .map(|(_, p)| p)
    }
}

impl Default for ProfileStore {
    fn default() -> Self {
        ProfileStore {
            profile_names: Store::default(),
            query: None,
            label: default_label().to_string(),
        }
    }
}

impl Length for ProfileStore {
    fn len(&self) -> usize {
        self.iter().count()
    }
}

fn default_label() -> &'static str {
    "Profiles"
}

fn default_config_path() -> Option<PathBuf> {
    home_dir().map(|p| p.join(".aws").join("config"))
}

fn profile_matches(profile: &ProfileName, query: Option<&Query>) -> bool {
    query
        .map(|q| profile.as_str().contains(q.word()))
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::aws::profile::{ProfileName, ProfileStore};

    const PROJECT_ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from_str(PROJECT_ROOT)
            .unwrap()
            .join("tests")
            .join("data")
            .join(name)
    }

    fn profile_name(s: &str) -> ProfileName {
        ProfileName::from(s)
    }

    #[test]
    fn test_from_file() {
        let path = fixture_path("aws_config");
        let store = ProfileStore::from_file(path);
        assert!(store.is_ok());

        let store = store.ok().unwrap();
        let mut iter = store.iter();
        assert_eq!(Some(&profile_name("default")), iter.next());
        assert_eq!(Some(&profile_name("inner-spaces")), iter.next());
        assert_eq!(Some(&profile_name("outer-spaces")), iter.next());
        assert_eq!(Some(&profile_name("profile-name")), iter.next());
        assert_eq!(Some(&profile_name("profile-inner-spaces")), iter.next());
        assert_eq!(Some(&profile_name("profile-outer-spaces")), iter.next());
        assert_eq!(None, iter.next());
    }
}
