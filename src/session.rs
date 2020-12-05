use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SessionId(u64);

impl SessionId {
    pub fn next(&self) -> SessionId {
        SessionId(self.0 + 1)
    }
}

impl From<u64> for SessionId {
    fn from(id: u64) -> Self {
        SessionId(id)
    }
}

pub struct Session {
    // client: CloudWatchLogsClient,
// groups: CwlGroupStore,
// streams: CwlStreamStore,
// events: CwlEventStore,
}

pub struct SessionSet {
    sessions: BTreeMap<SessionId, Session>,
    next_session_id: SessionId,
}

impl SessionSet {
    pub fn insert(&mut self, session: Session) {
        self.sessions.insert(self.next_session_id, session);
        self.next_session_id = self.next_session_id.next();
    }
}

impl Default for SessionSet {
    fn default() -> Self {
        SessionSet {
            sessions: BTreeMap::default(),
            next_session_id: SessionId(1),
        }
    }
}
