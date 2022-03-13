use std::fmt::Display;

use super::{type_error::TypeError, type_primitive::TypePrimitive, type_var::TypeVar};

#[derive(Debug, PartialEq, Clone)]
pub enum TypeEnum {
    Simple(TypePrimitive),
    Arrow(Box<TypeEnum>, Box<TypeEnum>),
    Var(TypeVar),
}

impl TypeEnum {
    fn used_tvars(&self) -> Vec<usize> {
        match self {
            TypeEnum::Var(v) => {
                vec![v.id]
            }
            TypeEnum::Arrow(a, b) => {
                // must maintain element order AND uniqueness
                let used = a.used_tvars();
                used.clone()
                    .into_iter()
                    .chain(b.used_tvars().into_iter().filter(|e| !used.contains(e)))
                    .collect::<Vec<usize>>()
            }
            _ => {
                vec![]
            }
        }
    }
}

impl Display for TypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tvars = self.used_tvars();
        write!(f, "{}", self.stringify_with_tvars(&tvars))
    }
}

impl TypeEnum {
    pub fn unify(&mut self, other: &mut Self) -> Result<(), TypeError> {
        match (&mut self.clone(), &mut other.clone()) {
            (a, b) if a == b => Ok(()),
            (TypeEnum::Var(tv), _) => match ((*tv.value).borrow_mut()).as_mut() {
                Some(ref mut x) => x.unify(other),
                None if other.contains(tv) => Err(TypeError::UnresolvableTypeVariable(
                    tv.clone(),
                    other.clone(),
                )),
                inner => {
                    *inner = Some(other.clone());
                    Ok(())
                }
            },
            (_, TypeEnum::Var(_)) => other.unify(self),
            (TypeEnum::Simple(TypePrimitive::Halt), TypeEnum::Simple(_))
            | (TypeEnum::Simple(_), TypeEnum::Simple(TypePrimitive::Halt)) => Ok(()),
            (TypeEnum::Simple(a), TypeEnum::Simple(b)) if a == b => Ok(()),
            (TypeEnum::Arrow(ref mut a1, ref mut a2), TypeEnum::Arrow(ref mut b1, ref mut b2)) => {
                a1.unify(b1).and(a2.unify(b2))
            }
            (a, b) => Err(TypeError::IrreconcilableTypes(a.clone(), b.clone())),
        }
    }

    pub fn contains(&self, other: &TypeVar) -> bool {
        match self {
            TypeEnum::Var(v) => {
                v == other
                    || if let Some(ref value) = **v.value.borrow() {
                        value.contains(other)
                    } else {
                        false
                    }
            }
            TypeEnum::Arrow(a, b) => a.contains(other) || b.contains(other),
            _ => false,
        }
    }

    fn stringify_with_tvars(&self, tvars: &Vec<usize>) -> String {
        format!(
            "{}",
            match self {
                TypeEnum::Simple(s) => format!("{}", s),
                TypeEnum::Arrow(a, b) if matches!(**a, TypeEnum::Arrow(..)) => format!(
                    "({}) -> {}",
                    a.stringify_with_tvars(tvars),
                    b.stringify_with_tvars(tvars)
                ),
                TypeEnum::Arrow(a, b) => format!(
                    "{} -> {}",
                    a.stringify_with_tvars(tvars),
                    b.stringify_with_tvars(tvars)
                ),
                TypeEnum::Var(v) => format!(
                    "'{}",
                    (tvars.iter().position(|x| x == &v.id).unwrap() as u8 + ('a' as u8)) as char
                ),
            }
        )
    }
}
