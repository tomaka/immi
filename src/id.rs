use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

lazy_static! {
    static ref NEXT_ID: AtomicUsize = AtomicUsize::new(1);
}

/// UNUSED, may be removed
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WidgetId(u64);

impl WidgetId {
    pub fn new() -> WidgetId {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        WidgetId(id as u64)
    }
}
