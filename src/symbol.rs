//! Symbol type representing an interned string identifier.

use std::fmt;

/// A symbol is a lightweight handle to an interned string.
/// Two symbols are equal if and only if they refer to the same interned string.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol {
    /// The index into the interner's string table.
    index: u32,
}

impl Symbol {
    /// Create a new symbol from an index.
    pub(crate) fn new(index: u32) -> Self {
        Self { index }
    }

    /// Returns the raw index of this symbol.
    pub fn as_u32(self) -> u32 {
        self.index
    }

    /// Returns the raw index as a usize.
    pub fn as_usize(self) -> usize {
        self.index as usize
    }

    /// A placeholder symbol for testing.
    pub fn placeholder() -> Self {
        Self { index: u32::MAX }
    }

    /// Check if this is the placeholder symbol.
    pub fn is_placeholder(self) -> bool {
        self.index == u32::MAX
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Symbol({})", self.index)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sym#{}", self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_creation() {
        let s = Symbol::new(0);
        assert_eq!(s.as_u32(), 0);
        assert_eq!(s.as_usize(), 0);
    }

    #[test]
    fn test_symbol_equality() {
        let a = Symbol::new(1);
        let b = Symbol::new(1);
        let c = Symbol::new(2);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_symbol_ordering() {
        let a = Symbol::new(1);
        let b = Symbol::new(2);
        assert!(a < b);
    }

    #[test]
    fn test_placeholder() {
        let p = Symbol::placeholder();
        assert!(p.is_placeholder());
        let s = Symbol::new(0);
        assert!(!s.is_placeholder());
    }

    #[test]
    fn test_symbol_display() {
        let s = Symbol::new(42);
        assert_eq!(format!("{}", s), "sym#42");
    }

    #[test]
    fn test_symbol_debug() {
        let s = Symbol::new(42);
        assert_eq!(format!("{:?}", s), "Symbol(42)");
    }

    #[test]
    fn test_symbol_copy() {
        let a = Symbol::new(5);
        let b = a;
        assert_eq!(a, b);
    }
}
