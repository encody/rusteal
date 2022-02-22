use std::rc::Rc;

use crate::{
    compilation_error::CompilationError,
    context::{CompilationBinding, CompilationContext, TypeContext},
    typing::{TypeEnum, TypeError},
    OP_SEPARATOR,
};

use super::{primitive::Primitive, Expression};

pub enum Bind {
    Let {
        identifier: String,
        value: Box<dyn Expression>,
        body: Box<dyn Expression>,
    },
    Const {
        identifier: String,
        value: Box<Primitive>,
        body: Box<dyn Expression>,
    },
}

impl Expression for Bind {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        let value_type = match self {
            Bind::Let { value, .. } => value.resolve(context),
            Bind::Const { value, .. } => value.resolve(context),
        }?;

        match self {
            Bind::Let {
                identifier, body, ..
            }
            | Bind::Const {
                identifier, body, ..
            } => {
                let context = TypeContext {
                    bind_scope: Rc::new(context.bind_scope.add(identifier.to_string(), value_type)),
                    global_scope: Rc::clone(&context.global_scope),
                    local_scope: Rc::clone(&context.local_scope),
                };
                body.resolve(&context)
            }
        }
    }

    fn compile(
        &self,
        context: &CompilationContext,
        _: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        match self {
            Bind::Const {
                identifier,
                value,
                body,
            } => {
                let value_compiled = value.compile(context, &mut Vec::new())?;
                let context = CompilationContext {
                    scope: context.scope.add(
                        identifier.to_string(),
                        CompilationBinding::Replacement(value_compiled),
                    ),
                    ..*context
                };
                Ok(body.compile(&context, &mut vec![])?)
            }
            Bind::Let {
                identifier,
                value,
                body,
            } => {
                let value_compiled = value.compile(context, &mut Vec::new())?;
                let scratch_id = context.scratch_id;
                let next_scratch_id = if context.scratch_id < u8::MAX {
                    context.scratch_id + 1
                } else {
                    return Err(CompilationError::OutOfScratchSpace);
                };
                let context = CompilationContext {
                    scope: context.scope.add(
                        identifier.to_string(),
                        CompilationBinding::ScratchVar(scratch_id),
                    ),
                    scratch_id: next_scratch_id,
                };
                let body_compiled = body.compile(&context, &mut vec![])?;
                Ok(
                    vec![value_compiled, format!("store {scratch_id}"), body_compiled]
                        .join(OP_SEPARATOR),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        context::TypeContext,
        expression::{
            apply::Apply,
            binary::Binary,
            primitive::Primitive,
            var::{Rvalue, Var},
            Expression,
        },
    };

    use super::Bind;

    #[test]
    fn test() {
        let e = Bind::Let {
            identifier: "x".to_string(),
            value: Box::new(Primitive::UInt64(5)),
            body: Box::new(Apply(
                Box::new(Apply(
                    Box::new(Binary::Equals),
                    Box::new(Primitive::UInt64(5)),
                )),
                Box::new(Rvalue(Var::Bind("x".to_string()))),
            )),
        };
        println!("{:?}", e.resolve(&TypeContext::default()));
        println!("{}", e.compile_raw().unwrap());
    }
}
