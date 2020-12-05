use crate::aws::cwlogs::group::{CwlGroupStore, RusotoCwlGroupCursor};
use crate::aws::profile::ProfileStore;
use crate::event::Key;
use crate::preset::{Preset, PresetName, PresetStore};
use crate::session::{Session, SessionSet};
use crate::ui::widget::debug::DebugData;
use crate::ui::widget::search::SearchData;

pub struct AppData {
    pub presets: PresetStore,
    pub profiles: ProfileStore,
    pub groups: CwlGroupStore,
    pub search: SearchData,
    sessions: SessionSet,
    debug: Option<DebugData>,
    groups_cursor: Option<RusotoCwlGroupCursor>,
}

impl AppData {
    pub fn new(
        presets: PresetStore,
        profiles: ProfileStore,
        groups: CwlGroupStore,
        search: SearchData,
        sessions: SessionSet,
        debug: Option<DebugData>,
    ) -> AppData {
        AppData {
            presets,
            profiles,
            groups,
            search,
            sessions,
            debug,
            groups_cursor: None,
        }
    }

    pub fn with_debug(debug: DebugData) -> AppData {
        // TODO: Load presets from file.
        let mut presets = PresetStore::default();
        presets.insert(Preset::new(PresetName::from("project-prd"), None));
        presets.insert(Preset::new(PresetName::from("project-stg"), None));
        presets.insert(Preset::new(PresetName::from("project-dev"), None));
        presets.insert(Preset::new(PresetName::from("lambda"), None));
        presets.insert(Preset::new(PresetName::from("glue"), None));

        let profiles = ProfileStore::from_shared_file().expect("could not load aws config");
        let groups = CwlGroupStore::default();
        let search = SearchData::default();
        AppData::new(
            presets,
            profiles,
            groups,
            search,
            SessionSet::default(),
            Some(debug),
        )
    }

    pub fn debug(&self) -> Option<&DebugData> {
        self.debug.as_ref()
    }

    pub fn debug_key(&mut self, key: Key) {
        if let Some(debug) = self.debug.as_mut() {
            debug.append_key(key);
        }
    }

    pub fn debug_log(&mut self, msg: String) {
        if let Some(debug) = self.debug.as_mut() {
            debug.append_log(msg);
        }
    }

    pub fn set_groups_cursor(&mut self, cursor: RusotoCwlGroupCursor) {
        self.groups_cursor = Some(cursor);
    }

    pub fn create_session(&mut self, session: Session) {
        self.sessions.insert(session);
    }
}

impl Default for AppData {
    fn default() -> Self {
        let mut presets = PresetStore::default();
        presets.insert(Preset::new(PresetName::from("project-prd"), None));
        presets.insert(Preset::new(PresetName::from("project-stg"), None));
        presets.insert(Preset::new(PresetName::from("project-dev"), None));
        presets.insert(Preset::new(PresetName::from("lambda"), None));
        presets.insert(Preset::new(PresetName::from("glue"), None));

        let profiles = ProfileStore::from_shared_file().expect("could not load aws config");
        let groups = CwlGroupStore::default();
        let search = SearchData::default();
        AppData::new(
            presets,
            profiles,
            groups,
            search,
            SessionSet::default(),
            None,
        )
    }
}
