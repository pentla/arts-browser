mod ast;
mod parse;

pub use crate::html::ast::{element_type, Element, ElementData, ElementType};
pub use crate::html::parse::parse_nodes;
