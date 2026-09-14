/*!
This module provides the main query engine implementation, as well as the parser for the query
language and the intermediary AST representations of queries.
*/
pub mod ast;
pub(crate) mod common;
pub mod dfa;
pub(crate) mod nfa;
pub mod parser;

// Re-exports
pub use ast::*;
pub use common::{JSONPointer, PathType};
pub use dfa::*;
pub use nfa::*;
pub use parser::*;
