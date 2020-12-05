use crate::aws::cwlogs::group::model::CwlGroup;
use crate::collection::store::Store;
use crate::collection::Length;

#[derive(Debug)]
pub struct CwlGroupStore {
    groups: Store<CwlGroup>,
}

impl CwlGroupStore {
    pub fn new(groups: Store<CwlGroup>) -> Self {
        CwlGroupStore { groups }
    }

    pub fn clear(&mut self) {
        self.groups.clear();
    }

    pub fn insert(&mut self, group: CwlGroup) {
        self.groups.insert(group);
    }

    pub fn extend(&mut self, groups: impl Iterator<Item = CwlGroup>) {
        self.groups.extend(groups);
    }

    pub fn order_by_name_asc(&self) -> impl Iterator<Item = &CwlGroup> {
        self.groups
            .order_by_asc_ref(|group| group.group_name.as_str())
    }

    pub fn order_by_creation_time_asc(&self) -> impl Iterator<Item = &CwlGroup> {
        self.groups.order_by_asc(|group| group.creation_time)
    }

    pub fn order_by_creation_time_desc(&self) -> impl Iterator<Item = &CwlGroup> {
        self.groups.order_by_desc(|group| group.creation_time)
    }

    pub fn order_by_size_asc(&self) -> impl Iterator<Item = &CwlGroup> {
        self.groups.order_by_asc(|group| group.stored)
    }

    pub fn order_by_size_desc(&self) -> impl Iterator<Item = &CwlGroup> {
        self.groups.order_by_desc(|group| group.stored)
    }
}

impl Default for CwlGroupStore {
    fn default() -> Self {
        CwlGroupStore::new(Store::default())
    }
}

impl Length for CwlGroupStore {
    fn len(&self) -> usize {
        self.groups.len()
    }
}
