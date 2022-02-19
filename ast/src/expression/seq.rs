use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    type_enum::{TypeEnum, TypeError, TypePrimitive},
    OP_SEPARATOR,
};

use super::Expression;

pub struct Seq(pub Vec<Box<dyn Expression>>);

impl Expression for Seq {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        self.0
            .last()
            .map_or(Ok(TypeEnum::Simple(TypePrimitive::Void)), |e| {
                e.resolve(context)
            })
    }

    fn compile(&self, context: &CompilationContext) -> Result<String, CompilationError> {
        let Self(expressions) = self;

        expressions
            .into_iter()
            .map(|e| e.compile(context))
            .collect::<Result<Vec<String>, CompilationError>>()
            .map(|s| s.join(OP_SEPARATOR))
    }
}
