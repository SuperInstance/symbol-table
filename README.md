# Symbol Table

A Rust implementation of a **lexical scope symbol table** for compilers and interpreters — tracking variable, function, struct, and enum declarations across nested scopes with proper shadowing semantics.

## Why It Matters

Every compiler and interpreter needs a symbol table. When the parser sees `let x = 5`, it must check: is `x` already defined in this scope? When the type checker sees `x + y`, it must resolve: what are the types of `x` and `y`? When codegen emits `load_local 0`, it must know: which slot is `x` in? The symbol table answers these questions by mapping names to their metadata (kind, type, scope, visibility) as the compiler traverses the AST. It handles the scoping rules that every programming language implements: global vs. local, block scoping, shadowing, closures, and module exports.

## How It Works

### Scope Stack

The symbol table is a tree of scopes:

```
Global scope (id=0)
    ├── Function "main" scope (id=1)
    │       ├── Block scope (id=2)
    │       └── Block scope (id=3)
    └── Function "helper" scope (id=4)
```

Each scope has a `parent` pointer for chain-of-scope lookup.

### Lookup — O(depth)

When resolving a name, search from the current scope upward:

```
lookup(name):
    scope = current
    while scope is not None:
        if name in scope.symbols: return symbol
        scope = scope.parent
    return None  // undefined
```

This naturally implements **shadowing**: an inner `x` hides an outer `x`.

### Define — O(1)

```
define(name, kind):
    if name in current_scope.symbols:
        return Err("already defined")
    current_scope.symbols[name] = Symbol { name, kind, scope: current }
```

Duplicate definitions in the **same** scope are errors. The same name in a **child** scope is allowed (shadowing).

### Symbol Kinds

```
SymbolKind:
    Variable { mutable: bool }
    Function { params: Vec<String> }
    Struct { fields: Vec<String> }
    Enum { variants: Vec<String> }
    Module
    TypeAlias
```

### Enter / Leave Scope

```
enter_scope() → new child scope, returns scope ID
leave_scope() → pop back to parent
```

### Complexity

| Operation | Time |
|-----------|------|
| define | O(1) |
| lookup | O(d) where d = scope depth |
| enter/leave scope | O(1) |
| list all symbols | O(n) total symbols |
| Space | O(n) for n symbols + O(s) for s scopes |

### Compared to Alternatives

- **Flat table with scope numbers**: Every symbol stores its scope ID; lookup filters by scope. Simpler but O(n) per lookup.
- **LeBlanc-Crowley**: Symbols stored in a per-scope linked list threaded through the symbol table. O(1) lookup with scope stack.
- This crate: HashMap-per-scope, O(1) define, O(d) lookup. Good balance of simplicity and performance.

## Quick Start

```rust
use symbol_table::{SymbolTable, SymbolKind};

fn main() {
    let mut st = SymbolTable::new();

    // Define a global variable
    st.define("pi", SymbolKind::Variable { mutable: false }).unwrap();

    // Enter function scope
    st.enter_scope();
    st.define("x", SymbolKind::Variable { mutable: true }).unwrap();

    // x is found in local scope
    assert!(st.lookup("x").is_some());
    // pi is found in global scope (via chain lookup)
    assert!(st.lookup("pi").is_some());

    // Leave function scope
    st.leave_scope();
    // x is no longer visible
    assert!(st.lookup("x").is_none());
    // pi is still there
    assert!(st.lookup("pi").is_some());
}
```

## API

### `SymbolTable`
- `new()` — create with a global scope (id=0)
- `enter_scope() -> usize` — push child scope, returns scope ID
- `leave_scope() -> Option<usize>` — pop to parent, returns parent ID
- `define(name, kind) -> Result<(), String>` — declare in current scope
- `lookup(name) -> Option<&Symbol>` — chain-of-scope resolution
- `lookup_mut(name) -> Option<&mut Symbol>` — mutable lookup
- `current_scope() -> usize`, `scope_depth() -> usize`
- `all_symbols() -> Vec<&Symbol>` — flatten all scopes

### `Symbol`
- `name: String`, `kind: SymbolKind`, `scope_id: usize`, `ty: Option<String>`, `exported: bool`

### `SymbolKind`
- `Variable { mutable }`, `Function { params }`, `Struct { fields }`, `Enum { variants }`, `Module`, `TypeAlias`

## Architecture Notes

In SuperInstance, the symbol table tracks fleet resource names and their binding scopes. Ship-local resources (variables, handlers) are defined in per-ship scopes; fleet-wide resources (shared keys, global configs) live in the root scope. The γ + η = C conservation engine uses scoped lookup to resolve resource references during policy evaluation. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Aho, A. et al. (2006). *Compilers: Principles, Techniques, and Tools*, 2nd ed., §2.7 (Symbol Tables). Addison-Wesley.
- Cooper, K. & Torczon, L. (2011). *Engineering a Compiler*, 2nd ed., §5.7. Morgan Kaufmann.
- Appel, A. (2004). *Modern Compiler Implementation in ML*, Ch. 5. Cambridge.

## License

MIT
