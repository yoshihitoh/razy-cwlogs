use crate::collection::Length;
use std::collections::{btree_set, BTreeSet, BinaryHeap};
use std::fmt::Debug;
use std::iter::FromIterator;

pub struct PeekIterator<T: Ord> {
    heap: BinaryHeap<T>,
}

impl<T: Ord> PeekIterator<T> {
    fn new(heap: BinaryHeap<T>) -> Self {
        PeekIterator { heap }
    }
}

impl<T: Ord> Iterator for PeekIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

#[derive(Debug)]
pub struct Store<T: Ord + Debug> {
    items: BTreeSet<T>,
}

impl<T: Ord + Debug> Store<T> {
    pub fn new(items: BTreeSet<T>) -> Self {
        Store { items }
    }

    pub fn with_items(items: BTreeSet<T>) -> Self {
        Store { items }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn insert(&mut self, item: T) {
        self.items.insert(item);
    }

    pub fn extend(&mut self, items: impl Iterator<Item = T>) {
        self.items.extend(items);
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn order_by_asc<U, F>(&self, f: F) -> impl Iterator<Item = &T>
    where
        U: Ord,
        F: Fn(&T) -> U,
    {
        let it = self.items.iter().map(|item| (f(item), item));
        BTreeSet::from_iter(it).into_iter().map(|(_, item)| item)
    }

    pub fn order_by_asc_ref<'a, U, F>(&'a self, f: F) -> impl Iterator<Item = &'a T>
    where
        U: 'a + Ord,
        F: Fn(&'a T) -> U,
    {
        let it = self.items.iter().map(|item| (f(item), item));
        BTreeSet::from_iter(it).into_iter().map(|(_, item)| item)
    }

    pub fn order_by_desc<U, F>(&self, f: F) -> impl Iterator<Item = &T>
    where
        U: Ord,
        F: Fn(&T) -> U,
    {
        let it = self.items.iter().map(|item| (f(item), item));
        let heap = BinaryHeap::from_iter(it);

        // cannot use BinaryHeap::into_iter_sorted because it's unstable. Use custom iterator.
        PeekIterator::new(heap).map(|(_, item)| item)
    }

    pub fn order_by_desc_ref<'a, U, F>(&'a self, f: F) -> impl Iterator<Item = &'a T>
    where
        U: 'a + Ord,
        F: Fn(&'a T) -> U,
    {
        let it = self.items.iter().map(|item| (f(item), item));
        let heap = BinaryHeap::from_iter(it);

        // cannot use BinaryHeap::into_iter_sorted because it's unstable. Use custom iterator.
        PeekIterator::new(heap).map(|(_, item)| item)
    }
}

impl<T: Ord + Debug> Default for Store<T> {
    fn default() -> Self {
        Store::new(BTreeSet::default())
    }
}

impl<T: Ord + Debug> FromIterator<T> for Store<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Store::with_items(BTreeSet::from_iter(iter))
    }
}

impl<T: Ord + Debug> IntoIterator for Store<T> {
    type Item = T;
    type IntoIter = btree_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T: Ord + Debug> Length for Store<T> {
    fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_by_asc() {
        let s = |s: &str| s.to_string();
        let store = Store::from_iter(vec![s("1"), s("9"), s("99"), s("10")].into_iter());

        // as integer
        let mut it = store.order_by_asc(|x| x.parse::<i32>().unwrap());
        assert_eq!(Some("1"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("9"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("10"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("99"), it.next().map(|s| s.as_str()));
        assert_eq!(None, it.next());

        // as &str
        let mut it = store.order_by_asc_ref(|x| x.as_str());
        assert_eq!(Some("1"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("10"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("9"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("99"), it.next().map(|s| s.as_str()));
        assert_eq!(None, it.next());
    }

    #[test]
    fn order_by_desc() {
        let s = |s: &str| s.to_string();
        let store = Store::from_iter(vec![s("1"), s("9"), s("99"), s("10")].into_iter());

        // as integer
        let mut it = store.order_by_desc(|x| x.parse::<i32>().unwrap());
        assert_eq!(Some("99"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("10"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("9"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("1"), it.next().map(|s| s.as_str()));
        assert_eq!(None, it.next());

        // as &str
        let mut it = store.order_by_desc_ref(|x| x.as_str());
        assert_eq!(Some("99"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("9"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("10"), it.next().map(|s| s.as_str()));
        assert_eq!(Some("1"), it.next().map(|s| s.as_str()));
        assert_eq!(None, it.next());
    }
}
