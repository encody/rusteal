use std::sync::atomic::{AtomicUsize, Ordering};

static LABEL_ID: AtomicUsize = AtomicUsize::new(0);

pub fn create_label_id() -> usize {
    LABEL_ID.fetch_add(1, Ordering::SeqCst)
}
