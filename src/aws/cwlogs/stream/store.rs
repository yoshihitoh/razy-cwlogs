use crate::aws::cwlogs::stream::model::CwlStream;
use crate::collection::store::Store;

#[derive(Debug)]
pub struct CwlStreamStore {
    streams: Store<CwlStream>,
}

impl CwlStreamStore {
    pub fn new(streams: Store<CwlStream>) -> Self {
        CwlStreamStore { streams }
    }

    pub fn insert(&mut self, stream: CwlStream) {
        self.streams.insert(stream);
    }

    pub fn extend(&mut self, streams: impl Iterator<Item = CwlStream>) {
        self.streams.extend(streams);
    }

    pub fn order_by_creation_time_asc(&self) -> impl Iterator<Item = &CwlStream> {
        self.streams.order_by_asc(|stream| stream.creation_time)
    }

    pub fn order_by_creation_time_desc(&self) -> impl Iterator<Item = &CwlStream> {
        self.streams.order_by_desc(|stream| stream.creation_time)
    }
}

impl Default for CwlStreamStore {
    fn default() -> Self {
        CwlStreamStore::new(Store::default())
    }
}
