use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typing::{TypeEnum, TypeError},
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
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError>;

    fn compile_raw(&self) -> Result<String, CompilationError> {
        self.compile(&CompilationContext::default(), &mut Vec::new())
    }
}
