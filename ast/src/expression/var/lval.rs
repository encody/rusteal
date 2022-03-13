use crate::{
    compilation_error::CompilationError,
    context::{CompilationBinding, CompilationContext, TypeContext},
    expression::{primitive::Primitive, Expression},
    typing::{TypeEnum, TypeError, TypePrimitive, TypeVar},
    OP_SEPARATOR, typesig,
};

use super::Var;

#[derive(Debug, Clone, PartialEq)]
pub struct LVal(pub Var);

impl Expression for LVal {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        let type_enum = self.0.get_type(context)?.clone();

        Ok(match self.0 {
            Var::Local(..) => typesig!(int -> #type_enum -> void),
            _ => typesig!(#type_enum -> void),
        })
    }

    fn compile(
        &self,
        context: &CompilationContext,
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        match &self.0 {
            Var::Bind(identifier) => {
                let what = prepared_stack.pop().ok_or(CompilationError::MissingStack)?;
                let scratch_binding =
                    context.scope.get(identifier).ok_or(CompilationError::from(
                        // should never happen if type checking is run before compilation
                        TypeError::UnboundIdentifier(self.0.clone()),
                    ))?;
                let scratch_id = if let CompilationBinding::ScratchVar(id) = scratch_binding {
                    id
                } else {
                    return Err(CompilationError::ConstantAssignment(
                        scratch_binding.to_owned(),
                    ));
                };
                Ok(format!("{what}{OP_SEPARATOR}store {scratch_id}"))
            }
            Var::Global(identifier) => {
                let what = prepared_stack.pop().ok_or(CompilationError::MissingStack)?;
                Ok(vec![
                    Primitive::from(identifier).compile(context, prepared_stack)?,
                    what,
                    "app_global_put".to_string(),
                ]
                .join(OP_SEPARATOR))
            }
            Var::Local(identifier) => {
                let who = prepared_stack.pop().ok_or(CompilationError::MissingStack)?;
                let what = prepared_stack.pop().ok_or(CompilationError::MissingStack)?;
                Ok(vec![
                    who,
                    Primitive::from(identifier).compile(context, prepared_stack)?,
                    what,
                    "app_local_put".to_string(),
                ]
                .join(OP_SEPARATOR))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        context::{CompilationBinding, CompilationContext, Scope, TypeContext},
        expression::{apply::Apply, primitive::Primitive, var::Var, Expr, Expression},
        typing::{TypeEnum, TypePrimitive},
    };

    use super::LVal;

    #[test]
    fn test_scratch() {
        let e = Apply(
            Expr::LVal(LVal(Var::Bind("key".to_string()))),
            Expr::Primitive(Primitive::Byteslice("value".as_bytes().to_vec())),
        );
        println!(
            "{:?}",
            e.resolve(&TypeContext {
                bind_scope: Rc::new(Scope::default().add(
                    "key".to_string(),
                    TypeEnum::Simple(TypePrimitive::Byteslice)
                )),
                ..Default::default()
            })
            .unwrap()
        );
        println!(
            "{}",
            e.compile(
                &CompilationContext {
                    scope: Scope::default()
                        .add("key".to_string(), CompilationBinding::ScratchVar(0)),
                    scratch_id: 1,
                },
                &mut vec![]
            )
            .unwrap()
        );
    }

    #[test]
    fn test_global() {
        let e = Apply(
            Expr::LVal(LVal(Var::Global("key".to_string()))),
            Expr::Primitive(Primitive::Byteslice("value".as_bytes().to_vec())),
        );
        println!(
            "{:?}",
            e.resolve(&TypeContext {
                global_scope: Rc::new(Scope::default().add(
                    "key".to_string(),
                    TypeEnum::Simple(TypePrimitive::Byteslice)
                )),
                ..Default::default()
            })
            .unwrap()
        );
        println!("{}", e.compile_raw().unwrap());
    }

    #[test]
    fn test_local() {
        let e = Apply(
            Expr::Apply(Box::new(Apply(
                Expr::LVal(LVal(Var::Local("key".to_string()))),
                Expr::Primitive(Primitive::UInt64(0)),
            ))),
            Expr::Primitive(Primitive::Byteslice("value".as_bytes().to_vec())),
        );
        println!(
            "{:?}",
            e.resolve(&TypeContext {
                local_scope: Rc::new(Scope::default().add(
                    "key".to_string(),
                    TypeEnum::Simple(TypePrimitive::Byteslice)
                )),
                ..Default::default()
            })
            .unwrap()
        );
        println!("{}", e.compile_raw().unwrap());
    }
}
