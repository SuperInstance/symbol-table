//! String interner for deduplicating string storage.

use crate::symbol::Symbol;
use std::collections::HashMap;

/// A string interner that stores unique strings and returns symbol handles.
pub struct StringInterner {
    strings: Vec<String>,
    map: HashMap<String, u32>,
}

impl StringInterner {
    /// Create a new empty interner.
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// Intern a string, returning its symbol. If the string was already
    /// interned, returns the existing symbol.
    pub fn intern(&mut self, s: &str) -> Symbol {
        if let Some(&idx) = self.map.get(s) {
            return Symbol::new(idx);
        }
        let idx = self.strings.len() as u32;
        self.strings.push(s.to_string());
        self.map.insert(s.to_string(), idx);
        Symbol::new(idx)
    }

    /// Get the string for a given symbol, if it exists.
    pub fn get(&self, sym: Symbol) -> Option<&str> {
        self.strings.get(sym.as_usize()).map(|s| s.as_str())
    }

    /// Returns the number of interned strings.
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Returns true if no strings have been interned.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Check if a string has been interned.
    pub fn contains(&self, s: &str) -> bool {
        self.map.contains_key(s)
    }

    /// Returns the symbol for an already-interned string.
    pub fn get_symbol(&self, s: &str) -> Option<Symbol> {
        self.map.get(s).map(|&idx| Symbol::new(idx))
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intern_new() {
        let mut interner = StringInterner::new();
        let sym = interner.intern("hello");
        assert_eq!(interner.get(sym), Some("hello"));
    }

    #[test]
    fn test_intern_dedup() {
        let mut interner = StringInterner::new();
        let a = interner.intern("foo");
        let b = interner.intern("foo");
        assert_eq!(a, b);
        assert_eq!(interner.len(), 1);
    }

    #[test]
    fn test_intern_different() {
        let mut interner = StringInterner::new();
        let a = interner.intern("foo");
        let b = interner.intern("bar");
        assert_ne!(a, b);
        assert_eq!(interner.len(), 2);
    }

    #[test]
    fn test_interner_empty() {
        let interner = StringInterner::new();
        assert!(interner.is_empty());
        assert_eq!(interner.len(), 0);
    }

    #[test]
    fn test_contains() {
        let mut interner = StringInterner::new();
        assert!(!interner.contains("test"));
        interner.intern("test");
        assert!(interner.contains("test"));
    }

    #[test]
    fn test_get_symbol() {
        let mut interner = StringInterner::new();
        let sym = interner.intern("x");
        assert_eq!(interner.get_symbol("x"), Some(sym));
        assert_eq!(interner.get_symbol("y"), None);
    }
}
