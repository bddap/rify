use core::fmt;
use std::collections::BTreeMap;

/// A mapping that keeps a history of writes. Writes to the map effect "pushes" to a stack. Those
/// "pushes" can be undone with a "pop".
pub struct MapStack<K, V> {
    current: BTreeMap<K, V>,
    history: Vec<(K, Option<V>)>,
}

impl<K: Ord, V> MapStack<K, V> {
    pub fn new() -> Self {
        Self {
            current: BTreeMap::new(),
            history: Vec::new(),
        }
    }
}

impl<K: Ord + Clone, V> MapStack<K, V> {
    /// Set a value appending to write history.
    pub fn write(&mut self, key: K, val: V) {
        let old_val = self.current.insert(key.clone(), val);
        self.history.push((key, old_val));
    }

    /// Undo most recent write or return error if there is no history to undo.
    pub fn undo(&mut self) -> Result<(), NoMoreHistory> {
        let (key, old_val) = self.history.pop().ok_or(NoMoreHistory)?;
        match old_val {
            Some(val) => self.current.insert(key, val),
            None => self.current.remove(&key),
        };
        Ok(())
    }
}

impl<K, V> AsRef<BTreeMap<K, V>> for MapStack<K, V> {
    fn as_ref(&self) -> &BTreeMap<K, V> {
        &self.current
    }
}

#[derive(Debug)]
pub struct NoMoreHistory;
impl fmt::Display for NoMoreHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attempted to pop from an empty stack.")
    }
}
impl std::error::Error for NoMoreHistory {}
