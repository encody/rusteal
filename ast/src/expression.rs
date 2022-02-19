use crate::{
    compilation_error::CompilationError,
    type_enum::{TypeError, TypeEnum}, context::{TypeContext, CompilationContext},
};

pub mod apply;
pub mod binary;
pub mod bind;
pub mod cond;
pub mod constant;
pub mod primitive;
pub mod ret;
pub mod seq;
pub mod txn;
pub mod var;

pub trait Expression {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError>;
    fn compile(&self, context: &CompilationContext) -> Result<String, CompilationError>;
}
