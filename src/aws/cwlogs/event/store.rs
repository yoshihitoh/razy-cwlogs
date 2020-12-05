use crate::aws::cwlogs::event::model::CwlEvent;
use crate::collection::store::Store;

#[derive(Debug)]
pub struct CwlEventStore {
    events: Store<CwlEvent>,
}

impl CwlEventStore {
    pub fn new(events: Store<CwlEvent>) -> Self {
        CwlEventStore { events }
    }

    pub fn insert(&mut self, event: CwlEvent) {
        self.events.insert(event);
    }

    pub fn extend(&mut self, events: impl Iterator<Item = CwlEvent>) {
        self.events.extend(events);
    }

    pub fn order_by_asc(&self) -> impl Iterator<Item = &CwlEvent> {
        self.events
            .order_by_asc_ref(|event| (event.event_time, &event.stream_name))
    }

    pub fn order_by_desc(&self) -> impl Iterator<Item = &CwlEvent> {
        self.events
            .order_by_desc_ref(|event| (event.event_time, &event.stream_name))
    }
}

impl Default for CwlEventStore {
    fn default() -> Self {
        CwlEventStore::new(Store::default())
    }
}
