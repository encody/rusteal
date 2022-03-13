use crate::{
    compilation_error::CompilationError,
    context::{CompilationBinding, CompilationContext, TypeContext},
    expression::{primitive::Primitive, Expression},
    typesig,
    typing::{TypeEnum, TypeError, TypePrimitive, TypeVar},
    OP_SEPARATOR,
};

use super::Var;

#[derive(Debug, Clone, PartialEq)]
pub struct RVal(pub Var);

impl Expression for RVal {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        let type_enum = self.0.get_type(context)?.clone();

        Ok(match self.0 {
            Var::Local(..) => typesig!(int -> #type_enum),
            _ => type_enum,
        })
    }

    fn compile(
        &self,
        context: &CompilationContext,
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        match &self.0 {
            Var::Global(identifier) => Ok(format!(
                "{push_identifier}{OP_SEPARATOR}app_global_get",
                push_identifier = Primitive::Byteslice(identifier.as_bytes().to_vec())
                    .compile(context, &mut Vec::new())?
            )),
            Var::Local(identifier) => Ok(format!(
                "{account}{OP_SEPARATOR}{push_identifier}{OP_SEPARATOR}app_local_get", // app_local_get pops 2 elements (second is account identifier), which is why it is typed as a function instead of a simple primitive
                account = prepared_stack.pop().ok_or(CompilationError::MissingStack)?,
                push_identifier = Primitive::Byteslice(identifier.as_bytes().to_vec())
                    .compile(context, &mut Vec::new())?
            )),
            Var::Bind(identifier) => {
                let binding = context.scope.get(&identifier).ok_or::<CompilationError>(
                    // should never happen if type checking is run before compilation
                    TypeError::UnboundIdentifier(self.0.clone()).into(),
                )?;

                Ok(match binding {
                    CompilationBinding::Replacement(s) => s.to_string(),
                    CompilationBinding::ScratchVar(i) => format!("load {i}"),
                })
            }
        }
    }
}
