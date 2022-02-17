use crate::{compilation_error::CompilationError, type_enum::{TypeEnum, TypeCheckError}};

pub mod primitive;
pub mod seq;
pub mod binary;
pub mod txn;
pub mod apply;

pub trait Expression {
  fn compile(&self) -> Result<String, CompilationError>;
  fn resolve(&self) -> Result<TypeEnum, TypeCheckError>;
}
