use std::collections::VecDeque;

pub mod store;

pub trait Length {
    fn len(&self) -> usize;
}

impl<T> Length for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl<T> Length for VecDeque<T> {
    fn len(&self) -> usize {
        VecDeque::len(self)
    }
}

pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for String {
    fn as_str(&self) -> &str {
        String::as_str(self)
    }
}
