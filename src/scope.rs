//! Scoped symbol management.

use crate::intern::StringInterner;
use crate::symbol::Symbol;
use std::collections::HashMap;

/// Metadata attached to a symbol in a scope.
#[derive(Clone, Debug, PartialEq)]
pub struct SymbolInfo {
    /// The interned symbol.
    pub symbol: Symbol,
    /// User-defined kind (e.g., variable, function, type).
    pub kind: u8,
    /// Optional depth level where defined.
    pub depth: usize,
}

/// A single scope level in a scope stack.
#[derive(Clone, Debug)]
pub struct Scope {
    /// Parent scope index, if any.
    parent: Option<usize>,
    depth: usize,
    bindings: HashMap<Symbol, SymbolInfo>,
    name_to_symbol: HashMap<String, Symbol>,
}

impl Scope {
    /// Create a new scope at the given depth with optional parent.
    pub fn new(depth: usize, parent: Option<usize>) -> Self {
        Self {
            parent,
            depth,
            bindings: HashMap::new(),
            name_to_symbol: HashMap::new(),
        }
    }

    /// Insert a binding into this scope.
    pub fn insert(&mut self, name: &str, symbol: Symbol, kind: u8) {
        let info = SymbolInfo {
            symbol,
            kind,
            depth: self.depth,
        };
        self.bindings.insert(symbol, info);
        self.name_to_symbol.insert(name.to_string(), symbol);
    }

    /// Look up a symbol by name in this scope only (no parent lookup).
    pub fn lookup_local(&self, name: &str) -> Option<&SymbolInfo> {
        self.name_to_symbol
            .get(name)
            .and_then(|sym| self.bindings.get(sym))
    }

    /// Look up a symbol by its Symbol handle in this scope.
    pub fn lookup_symbol(&self, sym: Symbol) -> Option<&SymbolInfo> {
        self.bindings.get(&sym)
    }

    /// Returns the depth of this scope.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the parent scope index.
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }

    /// Number of bindings in this scope.
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Returns true if this scope has no bindings.
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Returns an iterator over all bindings.
    pub fn bindings(&self) -> impl Iterator<Item = (&Symbol, &SymbolInfo)> {
        self.bindings.iter()
    }
}

/// A scope manager that maintains a stack of scopes.
pub struct ScopeManager {
    scopes: Vec<Scope>,
}

impl ScopeManager {
    /// Create a new scope manager with a global scope.
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(0, None)],
        }
    }

    /// Push a new scope onto the stack.
    pub fn push_scope(&mut self) -> usize {
        let depth = self.scopes.len();
        let parent = Some(depth - 1);
        self.scopes.push(Scope::new(depth, parent));
        depth
    }

    /// Pop the top scope. Returns false if at global scope.
    pub fn pop_scope(&mut self) -> bool {
        if self.scopes.len() <= 1 {
            return false;
        }
        self.scopes.pop();
        true
    }

    /// Insert a binding into the current (top) scope.
    pub fn define(&mut self, name: &str, kind: u8, interner: &mut StringInterner) -> Symbol {
        let sym = interner.intern(name);
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, sym, kind);
        }
        sym
    }

    /// Look up a name starting from the current scope, walking up parents.
    pub fn lookup(&self, name: &str, interner: &StringInterner) -> Option<&SymbolInfo> {
        let sym = interner.get_symbol(name)?;
        let mut scope_idx = self.scopes.len().checked_sub(1)?;
        loop {
            let scope = &self.scopes[scope_idx];
            if let Some(info) = scope.lookup_symbol(sym) {
                return Some(info);
            }
            match scope.parent() {
                Some(parent) => scope_idx = parent,
                None => return None,
            }
        }
    }

    /// Returns the current scope depth.
    pub fn current_depth(&self) -> usize {
        self.scopes.len().saturating_sub(1)
    }

    /// Number of scopes.
    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }
}

impl Default for ScopeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_new() {
        let scope = Scope::new(0, None);
        assert_eq!(scope.depth(), 0);
        assert!(scope.is_empty());
    }

    #[test]
    fn test_scope_insert_lookup() {
        let mut scope = Scope::new(0, None);
        let sym = Symbol::new(0);
        scope.insert("x", sym, 1);
        let info = scope.lookup_local("x").unwrap();
        assert_eq!(info.symbol, sym);
        assert_eq!(info.kind, 1);
    }

    #[test]
    fn test_scope_manager_push_pop() {
        let mut sm = ScopeManager::new();
        assert_eq!(sm.scope_count(), 1);
        sm.push_scope();
        assert_eq!(sm.scope_count(), 2);
        sm.push_scope();
        assert_eq!(sm.scope_count(), 3);
        assert!(sm.pop_scope());
        assert_eq!(sm.scope_count(), 2);
    }

    #[test]
    fn test_scope_manager_cannot_pop_global() {
        let mut sm = ScopeManager::new();
        assert!(!sm.pop_scope());
    }

    #[test]
    fn test_scope_lookup_through_parents() {
        let mut sm = ScopeManager::new();
        let mut interner = StringInterner::new();
        sm.define("x", 1, &mut interner);
        sm.push_scope();
        sm.push_scope();
        let info = sm.lookup("x", &interner).unwrap();
        assert_eq!(info.depth, 0);
    }
}
