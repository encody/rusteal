use std::rc::Rc;

use crate::type_enum::TypeEnum;

#[derive(Clone)]
pub struct Scope<'a, K: PartialEq, V> {
    item: Option<(K, V)>,
    parent: Option<&'a Scope<'a, K, V>>,
}

impl<'a, K: PartialEq, V> Default for Scope<'a, K, V> {
    fn default() -> Self {
        Self {
            item: None,
            parent: None,
        }
    }
}

impl<'a, K: PartialEq, V> Scope<'a, K, V> {
    pub fn get(&self, search: &K) -> Option<&V> {
        match self {
            Scope {
                item: Some((k, v)), ..
            } if k == search => Some(v),
            Scope {
                parent: Some(parent),
                ..
            } => parent.get(search),
            _ => None,
        }
    }

    pub fn add(&'a self, k: K, v: V) -> Self {
        Self {
            item: Some((k, v)),
            parent: Some(self),
        }
    }
}

#[derive(Default)]
pub struct TypeContext<'a> {
    pub bind_scope: Rc<Scope<'a, String, TypeEnum>>,
    pub global_scope: Rc<Scope<'a, String, TypeEnum>>,
    pub local_scope: Rc<Scope<'a, String, TypeEnum>>,
}

#[derive(Default)]
pub struct CompilationContext<'a> {
    pub scope: Scope<'a, String, CompilationBinding>,
    pub scratch_id: u8,
}

#[derive(Debug, Clone)]
pub enum CompilationBinding {
    ScratchVar(u8),
    Replacement(String),
}

#[cfg(test)]
mod tests {
    use super::Scope;

    #[test]
    fn scope() {
        let s = Scope::<&str, &str>::default();
        let s = s.add("k", "a");
        {
            let s = s.add("k", "b");
            {
                let s = s.add("k", "c");
                println!("{:?}", s.get(&"k"));
            }
            println!("{:?}", s.get(&"k"));
        }
        println!("{:?}", s.get(&"k"));
    }
}
