//! # Symbol Table
//!
//! A symbol table implementation with hash consing, interning, and scope management
//! for compiler infrastructure.

pub mod hash_cons;
pub mod intern;
pub mod scope;
pub mod symbol;
pub mod table;

pub use hash_cons::HashCons;
pub use intern::StringInterner;
pub use scope::Scope;
pub use symbol::Symbol;
pub use table::SymbolTable;
