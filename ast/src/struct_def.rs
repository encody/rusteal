use std::collections::HashMap;

use crate::type_enum::TypePrimitive;

#[derive(Default)]
pub struct StructDef<'a> {
    pub fields: HashMap<&'a str, TypePrimitive>,
}
