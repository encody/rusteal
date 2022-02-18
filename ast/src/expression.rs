use crate::{compilation_error::CompilationError, type_enum::{TypeEnum, TypeCheckError}};

pub mod apply;
pub mod binary;
pub mod cond;
pub mod constant;
pub mod primitive;
pub mod ret;
pub mod seq;
pub mod txn;

pub trait Expression {
  fn compile(&self) -> Result<String, CompilationError>;
  fn resolve(&self) -> Result<TypeEnum, TypeCheckError>;
}
