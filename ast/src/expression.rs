use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    type_enum::{TypeEnum, TypeError},
    OP_SEPARATOR,
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
    fn compile(
        &self,
        context: &CompilationContext,
        prepared_stack: Option<String>,
    ) -> Result<String, CompilationError>;
}

pub fn prepend_stack(prepared_stack: Option<String>) -> impl Fn(String) -> String {
    move |compiled_expression: String| {
        if let Some(s) = &prepared_stack {
            format!("{s}{OP_SEPARATOR}{compiled_expression}")
        } else {
            compiled_expression
        }
    }
}
