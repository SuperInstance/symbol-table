# symbol-table

Symbol table implementation with hash consing, interning, and scope management for compiler infrastructure.

## Features

- **String interning** — Deduplicate string storage with `StringInterner`
- **Hash consing** — Structural sharing via `HashCons` table
- **Scope management** — Nested scopes with parent chain lookup and shadowing
- **Symbol table** — High-level API combining interning and scoped lookup
- **Zero dependencies** — Pure `std` implementation

## Usage

```rust
use symbol_table::SymbolTable;

let mut st = SymbolTable::new();
st.define_variable("x");
st.push_scope();
st.define_variable("x"); // shadows outer x
st.pop_scope();
```

## License

MIT
