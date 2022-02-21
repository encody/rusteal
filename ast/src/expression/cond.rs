use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    label::create_label_id,
    type_enum::{TypeEnum, TypeError, TypePrimitive},
    OP_SEPARATOR,
};

use super::{Expression, prepend_stack};

pub struct Cond(
    pub Box<dyn Expression>,
    pub Box<dyn Expression>,
    pub Option<Box<Self>>,
);

impl Expression for Cond {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        let Cond(test, body, continuation) = self;

        test.resolve(context)?
            .unify(&mut TypeEnum::Simple(TypePrimitive::UInt64))?;
        let mut body_type = body.resolve(context)?;
        if let Some(ref c) = continuation {
            let mut continuation_type = c.resolve(context)?;
            body_type.unify(&mut continuation_type)?;
        }

        Ok(body_type)
    }

    fn compile(&self, context: &CompilationContext, prepared_stack: Option<String>) -> Result<String, CompilationError> {
        let label_id = format!("cond{}", create_label_id());
        let Cond(test, body, continuation) = self;

        let continuation = if let Some(c) = continuation {
            c.compile(context, None)?
        } else {
            "err".to_string()
        };

        let pieces = vec![
            test.compile(context, None)?,
            format!("bnz {label_id}"),
            continuation,
            format!("{label_id}:"),
            body.compile(context, None)?,
        ];

        Ok(pieces.join(OP_SEPARATOR)).map(prepend_stack(prepared_stack))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        context::{CompilationContext, TypeContext},
        expression::{cond::Cond, primitive::Primitive, Expression},
    };

    #[test]
    fn test() {
        let prog = Cond(
            Box::new(Primitive::UInt64(0)),
            Box::new(Primitive::Byteslice(b"hello".to_vec())),
            Some(Box::new(Cond(
                Box::new(Primitive::UInt64(1)),
                Box::new(Primitive::UInt64(6)),
                None,
            ))),
        );
        println!("{:?}", prog.resolve(&TypeContext::default()));
        println!("{}", prog.compile(&CompilationContext::default(), None).unwrap());
    }
}
