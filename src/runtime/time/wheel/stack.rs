use crate::time::timestamp::Timestamp;

/// Abstracts the stack operations needed to track timeouts.
pub(crate) trait Stack: Default {
    /// Items stored in stack
    type Value;

    /// Item storage, this allows a slab to be used instead of just the heap
    type Store;

    /// Returns `true` if the stack is empty
    fn is_empty(&self) -> bool;

    /// Push an item onto the stack
    fn push(&mut self, item: Self::Value, store: &mut Self::Store);

    /// Pop an item from the stack
    fn pop(&mut self, store: &mut Self::Store) -> Option<Self::Value>;

    fn remove(&mut self, item: &Self::Value, store: &mut Self::Store);

    fn when(item: &Self::Value, store: &Self::Store) -> Timestamp;
}
