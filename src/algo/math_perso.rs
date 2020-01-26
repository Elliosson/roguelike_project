use std::sync::atomic::{AtomicUsize, Ordering};

pub fn add_or_zero(u: u32, i: i32) -> u32 {
    if (u as i32 + i) < 0 {
        return 0;
    } else {
        return (u as i32 + i) as u32;
    }
}

//generate an unique id Id
pub fn _unique_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
