use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    type_enum::{TypeEnum, TypeError},
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

    fn compile(
        &self,
        context: &CompilationContext,
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        let arg = self.1.compile(context, prepared_stack)?;
        prepared_stack.push(arg);
        let f = self.0.compile(context, prepared_stack)?;
        Ok(f)
    }
}
