use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use super::type_enum::TypeEnum;

#[derive(Debug, Clone)]
pub struct TypeVar {
    pub id: usize,
    pub value: Rc<RefCell<Box<Option<TypeEnum>>>>,
}

impl PartialEq for TypeVar {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

static TYPE_VAR_ID: AtomicUsize = AtomicUsize::new(0);

impl TypeVar {
    pub fn new() -> Self {
        Self {
            id: TYPE_VAR_ID.fetch_add(1, Ordering::SeqCst),
            value: Rc::new(RefCell::new(Box::new(None))),
        }
    }
}
