//! Hash consing for structural sharing of values.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// A hash-consed value identifier.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct HCValue(u64);

impl HCValue {
    fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the raw id.
    pub fn id(self) -> u64 {
        self.0
    }
}

/// A hash cons table that ensures structural equality implies pointer equality.
/// Stores serialized representations and deduplicates via hashing.
pub struct HashCons {
    entries: Vec<Vec<u8>>,
    map: HashMap<u64, u32>,
    counter: u64,
}

impl HashCons {
    /// Create a new empty hash cons table.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            map: HashMap::new(),
            counter: 0,
        }
    }

    /// Insert raw bytes into the hash cons table.
    /// Returns a handle that is equal for identical bytes.
    pub fn cons(&mut self, data: &[u8]) -> HCValue {
        let hash = Self::compute_hash(data);
        self.counter += 1;
        if let Some(&idx) = self.map.get(&hash) {
            return HCValue::new(idx as u64);
        }
        let idx = self.entries.len() as u32;
        self.entries.push(data.to_vec());
        self.map.insert(hash, idx);
        HCValue::new(idx as u64)
    }

    /// Get the data for a hash-consed value.
    pub fn get(&self, val: HCValue) -> Option<&[u8]> {
        self.entries.get(val.id() as usize).map(|v| v.as_slice())
    }

    /// Number of unique entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Total number of cons operations (including duplicates).
    pub fn total_operations(&self) -> u64 {
        self.counter
    }

    fn compute_hash(data: &[u8]) -> u64 {
        // Simple FNV-1a-style hash using std hasher
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for HashCons {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cons_new() {
        let mut hc = HashCons::new();
        let v = hc.cons(b"hello");
        assert_eq!(hc.get(v), Some(&b"hello"[..]));
    }

    #[test]
    fn test_cons_dedup() {
        let mut hc = HashCons::new();
        let a = hc.cons(b"data");
        let b = hc.cons(b"data");
        assert_eq!(a, b);
        assert_eq!(hc.len(), 1);
        assert_eq!(hc.total_operations(), 2); // counter increments even on dedup
    }

    #[test]
    fn test_cons_different() {
        let mut hc = HashCons::new();
        let a = hc.cons(b"aaa");
        let b = hc.cons(b"bbb");
        assert_ne!(a, b);
        assert_eq!(hc.len(), 2);
    }

    #[test]
    fn test_hash_cons_empty() {
        let hc = HashCons::new();
        assert!(hc.is_empty());
    }
}
