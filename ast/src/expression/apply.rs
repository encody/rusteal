use crate::{
    compilation_error::CompilationError,
    type_enum::{TypeCheckError, TypeEnum},
    OP_SEPARATOR,
};

use super::Expression;

pub struct Apply(pub Box<dyn Expression>, pub Box<dyn Expression>);

impl Expression for Apply {
    fn resolve(&self) -> Result<TypeEnum, TypeCheckError> {
        let mut f_type = (*self.0).resolve()?;
        let mut arg_type = (*self.1).resolve()?;
        match f_type {
            TypeEnum::Arrow(ref mut param_type, body_type) => {
                param_type.unify(&mut arg_type)?;
                Ok(*body_type)
            }
            _ => Err(TypeCheckError::NonFunctionApplication(f_type)),
        }
    }

    fn compile(&self) -> Result<String, CompilationError> {
        let f = self.0.compile()?;
        let arg = self.1.compile()?;
        Ok(format!("{arg}{OP_SEPARATOR}{f}"))
    }
}
