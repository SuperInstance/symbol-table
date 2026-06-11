use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable { mutable: bool },
    Function { params: Vec<String> },
    Struct { fields: Vec<String> },
    Enum { variants: Vec<String> },
    Module,
    TypeAlias,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub scope_id: usize,
    pub ty: Option<String>,
    pub exported: bool,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub id: usize,
    pub parent: Option<usize>,
    pub symbols: HashMap<String, Symbol>,
}

pub struct SymbolTable {
    scopes: Vec<Scope>,
    current: usize,
    counter: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        let global = Scope { id: 0, parent: None, symbols: HashMap::new() };
        Self { scopes: vec![global], current: 0, counter: 1 }
    }

    pub fn enter_scope(&mut self) -> usize {
        let id = self.counter;
        self.counter += 1;
        self.scopes.push(Scope { id, parent: Some(self.current), symbols: HashMap::new() });
        self.current = id;
        id
    }

    pub fn leave_scope(&mut self) -> Option<usize> {
        let parent = self.scopes[self.current].parent;
        if let Some(p) = parent { self.current = p; }
        parent
    }

    pub fn define(&mut self, name: &str, kind: SymbolKind) -> Result<(), String> {
        let scope = &mut self.scopes[self.current];
        if scope.symbols.contains_key(name) {
            return Err(format!("Symbol '{}' already defined in this scope", name));
        }
        scope.symbols.insert(name.to_string(), Symbol {
            name: name.to_string(), kind, scope_id: self.current, ty: None, exported: false,
        });
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut scope_id = Some(self.current);
        while let Some(sid) = scope_id {
            if let Some(sym) = self.scopes[sid].symbols.get(name) { return Some(sym); }
            scope_id = self.scopes[sid].parent;
        }
        None
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        let mut scope_id = Some(self.current);
        while let Some(sid) = scope_id {
            if self.scopes[sid].symbols.contains_key(name) {
                return self.scopes[sid].symbols.get_mut(name);
            }
            scope_id = self.scopes[sid].parent;
        }
        None
    }

    pub fn current_scope(&self) -> usize { self.current }
    pub fn scope_depth(&self) -> usize {
        let mut depth = 0;
        let mut sid = Some(self.current);
        while let Some(id) = sid {
            if let Some(p) = self.scopes[id].parent { depth += 1; sid = Some(p); }
            else { break; }
        }
        depth
    }

    pub fn all_symbols(&self) -> Vec<&Symbol> {
        self.scopes.iter().flat_map(|s| s.symbols.values()).collect()
    }
}

impl Default for SymbolTable { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_define_lookup() {
        let mut st = SymbolTable::new();
        st.define("x", SymbolKind::Variable { mutable: false }).unwrap();
        assert!(st.lookup("x").is_some());
    }
    #[test]
    fn test_scope_nesting() {
        let mut st = SymbolTable::new();
        st.define("x", SymbolKind::Variable { mutable: false }).unwrap();
        st.enter_scope();
        st.define("y", SymbolKind::Variable { mutable: true }).unwrap();
        assert!(st.lookup("x").is_some());
        assert!(st.lookup("y").is_some());
        st.leave_scope();
        assert!(st.lookup("y").is_none());
    }
}
