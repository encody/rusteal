use crate::{
    compilation_error::CompilationError, expression::Expression, type_enum::TypeCheckError,
    OP_SEPARATOR,
};

pub struct Program {
    pub version: u64,
    pub body: Box<dyn Expression>,
}

impl Program {
    pub fn type_check(&self) -> Result<(), TypeCheckError> {
        let resolution = (*self.body).resolve()?;
        println!("{:?}", resolution);
        Ok(())
    }

    pub fn compile(&self) -> Result<String, CompilationError> {
        let version = self.version;
        self.body
            .compile()
            .map(|compiled| format!("#pragma version {version}{OP_SEPARATOR}{compiled}"))
    }
}
