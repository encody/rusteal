use std::collections::HashMap;

use crate::typing::TypePrimitive;

#[derive(Default)]
pub struct StructDef<'a> {
    pub fields: HashMap<&'a str, TypePrimitive>,
}
