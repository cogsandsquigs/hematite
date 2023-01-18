use std::cmp::{Ord, Ordering, PartialOrd};

/// An item in a heap.
#[derive(Clone, PartialEq, Eq)]
pub struct HeapItem<T> {
    /// The item's value.
    pub value: T,

    /// The item's priority.
    priority: i32,
}

impl<T> HeapItem<T> {
    /// Create a new heap item.
    pub fn new(value: T, priority: i32) -> Self {
        Self { value, priority }
    }
}

impl<T> PartialOrd for HeapItem<T>
where
    T: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl<T> Ord for HeapItem<T>
where
    T: PartialEq + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}
