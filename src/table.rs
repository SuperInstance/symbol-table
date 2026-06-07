//! High-level symbol table combining interning and scopes.

use crate::intern::StringInterner;
use crate::scope::{ScopeManager, SymbolInfo};
use crate::symbol::Symbol;

/// Symbol kind constants.
pub const KIND_VARIABLE: u8 = 0;
pub const KIND_FUNCTION: u8 = 1;
pub const KIND_TYPE: u8 = 2;
pub const KIND_CONSTANT: u8 = 3;

/// A full symbol table with interning and scoped lookup.
pub struct SymbolTable {
    interner: StringInterner,
    scopes: ScopeManager,
}

impl SymbolTable {
    /// Create a new symbol table with a global scope.
    pub fn new() -> Self {
        Self {
            interner: StringInterner::new(),
            scopes: ScopeManager::new(),
        }
    }

    /// Define a variable symbol in the current scope.
    pub fn define_variable(&mut self, name: &str) -> Symbol {
        self.scopes.define(name, KIND_VARIABLE, &mut self.interner)
    }

    /// Define a function symbol in the current scope.
    pub fn define_function(&mut self, name: &str) -> Symbol {
        self.scopes.define(name, KIND_FUNCTION, &mut self.interner)
    }

    /// Define a type symbol in the current scope.
    pub fn define_type(&mut self, name: &str) -> Symbol {
        self.scopes.define(name, KIND_TYPE, &mut self.interner)
    }

    /// Define a constant symbol in the current scope.
    pub fn define_constant(&mut self, name: &str) -> Symbol {
        self.scopes.define(name, KIND_CONSTANT, &mut self.interner)
    }

    /// Look up a symbol by name, searching from current scope upward.
    pub fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        self.scopes.lookup(name, &self.interner)
    }

    /// Get the string for a symbol handle.
    pub fn get_name(&self, sym: Symbol) -> Option<&str> {
        self.interner.get(sym)
    }

    /// Push a new scope.
    pub fn push_scope(&mut self) {
        self.scopes.push_scope();
    }

    /// Pop the current scope.
    pub fn pop_scope(&mut self) -> bool {
        self.scopes.pop_scope()
    }

    /// Current scope depth (0 = global).
    pub fn depth(&self) -> usize {
        self.scopes.current_depth()
    }

    /// Number of interned strings.
    pub fn symbol_count(&self) -> usize {
        self.interner.len()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_lookup_variable() {
        let mut st = SymbolTable::new();
        let sym = st.define_variable("x");
        let info = st.lookup("x").unwrap();
        assert_eq!(info.symbol, sym);
        assert_eq!(info.kind, KIND_VARIABLE);
    }

    #[test]
    fn test_shadowing() {
        let mut st = SymbolTable::new();
        st.define_variable("x");
        st.push_scope();
        let inner = st.define_variable("x");
        let info = st.lookup("x").unwrap();
        assert_eq!(info.symbol, inner);
        assert_eq!(info.depth, 1);
    }

    #[test]
    fn test_scope_pop_restores() {
        let mut st = SymbolTable::new();
        let outer = st.define_variable("y");
        st.push_scope();
        st.define_variable("y");
        st.pop_scope();
        let info = st.lookup("y").unwrap();
        assert_eq!(info.symbol, outer);
        assert_eq!(info.depth, 0);
    }

    #[test]
    fn test_get_name() {
        let mut st = SymbolTable::new();
        let sym = st.define_function("main");
        assert_eq!(st.get_name(sym), Some("main"));
    }

    #[test]
    fn test_multiple_kinds() {
        let mut st = SymbolTable::new();
        st.define_variable("x");
        st.define_function("foo");
        st.define_type("MyType");
        st.define_constant("PI");
        assert_eq!(st.symbol_count(), 4);
    }

    #[test]
    fn test_lookup_not_found() {
        let st = SymbolTable::new();
        assert!(st.lookup("nonexistent").is_none());
    }
}
