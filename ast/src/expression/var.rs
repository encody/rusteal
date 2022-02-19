use crate::{
    compilation_error::CompilationError,
    context::{CompilationBinding, CompilationContext, TypeContext},
    type_enum::{TypeEnum, TypeError},
};

use super::Expression;

pub struct Var(pub String);

impl Expression for Var {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        let Var(identifier) = self;

        context
            .scope
            .get(identifier)
            .ok_or(TypeError::UnboundIdentifier(identifier.to_string()))
            .map(|t| t.to_owned())
    }

    fn compile(&self, context: &CompilationContext) -> Result<String, CompilationError> {
        let Var(identifier) = self;

        let binding = context.scope.get(identifier).ok_or::<CompilationError>(
            // should never happen if type checking is run before compilation
            TypeError::UnboundIdentifier(identifier.to_string()).into(),
        )?;

        Ok(match binding {
            CompilationBinding::Replacement(s) => s.to_string(),
            CompilationBinding::ScratchVar(i) => format!("load {i}"),
        })
    }
}
