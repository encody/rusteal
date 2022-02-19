use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    type_enum::{TypeEnum, TypeError, TypePrimitive},
};

use super::Expression;

#[derive(Debug)]
pub enum OnComplete {
    NoOp,
    OptIn,
    CloseOut,
    ClearState,
    UpdateApplication,
    DeleteApplication,
}

impl Expression for OnComplete {
    fn resolve(&self, _: &TypeContext) -> Result<TypeEnum, TypeError> {
        Ok(TypeEnum::Simple(TypePrimitive::UInt64))
    }

    fn compile(&self, _: &CompilationContext) -> Result<String, CompilationError> {
        Ok(format!("int {self:?}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{expression::{constant::OnComplete, Expression}, context::CompilationContext};

    #[test]
    fn test() {
        let e = OnComplete::NoOp;
        assert_eq!(e.compile(&CompilationContext::default()).unwrap(), "int NoOp");
    }
}
