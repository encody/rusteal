use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    expression::{primitive::Primitive, Expression},
    type_enum::TypeError,
    MAX_TEAL_VERSION, OP_SEPARATOR,
};

pub struct Program {
    pub version: u64,
    pub body: Box<dyn Expression>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            version: MAX_TEAL_VERSION,
            body: Box::new(Primitive::UInt64(0)),
        }
    }
}

impl Program {
    pub fn type_check(&self) -> Result<(), TypeError> {
        let resolution = (*self.body).resolve(&TypeContext::default())?;
        println!("{:?}", resolution);
        Ok(())
    }

    pub fn compile(&self) -> Result<String, CompilationError> {
        let version = self.version;
        self.body
            .compile(&CompilationContext::default(), None)
            .map(|compiled| format!("#pragma version {version}{OP_SEPARATOR}{compiled}"))
    }
}
