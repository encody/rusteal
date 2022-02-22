use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TypePrimitive {
    Void,
    UInt64,
    Byteslice,
    Halt,
}

impl Display for TypePrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TypePrimitive::Void => "<void>",
                TypePrimitive::UInt64 => "int",
                TypePrimitive::Byteslice => "bytes",
                TypePrimitive::Halt => "<halt>",
            }
        )
    }
}
