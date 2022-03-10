use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typing::{TypeEnum, TypeError, TypePrimitive},
    OP_SEPARATOR,
};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Ret;

impl Expression for Ret {
    fn resolve(&self, _: &TypeContext) -> Result<TypeEnum, TypeError> {
        Ok(TypeEnum::Arrow(
            Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
            Box::new(TypeEnum::Simple(TypePrimitive::Halt)),
        ))
    }

    fn compile(
        &self,
        _context: &CompilationContext,
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        Ok(format!(
            "{}{OP_SEPARATOR}return",
            prepared_stack.pop().ok_or(CompilationError::MissingStack)?
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        context::TypeContext,
        expression::{ret::Ret, Expression},
    };

    #[test]
    fn test() {
        // let e = Ret::Approve;
        // println!("{:?}", e.resolve(&TypeContext::default()));
        // println!("{}", e.compile_raw().unwrap());
    }
}
