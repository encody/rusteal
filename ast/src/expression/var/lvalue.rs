use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    expression::{primitive::Primitive, Expression},
    type_enum::{TypeEnum, TypeError, TypePrimitive},
    OP_SEPARATOR,
};

use super::Var;

pub struct Lvalue(pub Var);

impl Expression for Lvalue {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        let type_enum = self.0.get_type(context)?;

        let assign = TypeEnum::Arrow(
            Box::new(type_enum.clone()),
            Box::new(TypeEnum::Simple(TypePrimitive::Void)),
        );

        Ok(match self.0 {
            Var::Local(..) => TypeEnum::Arrow(
                Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
                Box::new(assign),
            ),
            _ => assign,
        })
    }

    fn compile(
        &self,
        context: &CompilationContext,
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        match &self.0 {
            Var::Local(identifier) => {
                let who = prepared_stack.pop().ok_or(CompilationError::MissingStack)?;
                let what = prepared_stack.pop().ok_or(CompilationError::MissingStack)?;
                Ok(vec![
                    who,
                    Primitive::Byteslice(identifier.as_bytes().to_vec())
                        .compile(context, prepared_stack)?,
                    what,
                    "app_local_put".to_string(),
                ]
                .join(OP_SEPARATOR))
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        context::{CompilationContext, Scope, TypeContext},
        expression::{apply::Apply, primitive::Primitive, var::Var, Expression},
        type_enum::{TypeEnum, TypePrimitive},
    };

    use super::Lvalue;

    #[test]
    fn test() {
        let e = Apply(
            Box::new(Apply(
                Box::new(Lvalue(Var::Local("key".to_string()))),
                Box::new(Primitive::UInt64(0)),
            )),
            Box::new(Primitive::Byteslice("value".as_bytes().to_vec())),
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
