use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    expression::Expression,
    type_enum::TypeError,
    OP_SEPARATOR,
};

pub struct Program {
    pub version: u64,
    pub body: Box<dyn Expression>,
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
            .compile(&CompilationContext::default())
            .map(|compiled| format!("#pragma version {version}{OP_SEPARATOR}{compiled}"))
    }
}
