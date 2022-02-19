use crate::{
    compilation_error::CompilationError,
    context::{TypeContext, CompilationContext},
    type_enum::{TypeError, TypeEnum},
    OP_SEPARATOR,
};

use super::Expression;

pub struct Apply(pub Box<dyn Expression>, pub Box<dyn Expression>);

impl Expression for Apply {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        let mut f_type = (*self.0).resolve(context)?;
        let mut arg_type = (*self.1).resolve(context)?;
        match f_type {
            TypeEnum::Arrow(ref mut param_type, body_type) => {
                param_type.unify(&mut arg_type)?;
                Ok(*body_type)
            }
            _ => Err(TypeError::NonFunctionApplication(f_type)),
        }
    }

    fn compile(&self, context: &CompilationContext) -> Result<String, CompilationError> {
        let f = self.0.compile(context)?;
        let arg = self.1.compile(context)?;
        Ok(format!("{arg}{OP_SEPARATOR}{f}"))
    }
}
