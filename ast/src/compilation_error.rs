use crate::{context::CompilationBinding, typing::TypeError};
use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilationError {
    #[error("Type checking failed")]
    TypeCheck(#[from] TypeError),
    #[error("Error parsing bytes string")]
    BytesStringParse(#[from] FromUtf8Error),
    #[error("Out of scratch space")]
    OutOfScratchSpace,
    #[error("Missing stack")]
    MissingStack,
    #[error("Attempt to assign to constant expression: {0:?}")]
    ConstantAssignment(CompilationBinding),
}
