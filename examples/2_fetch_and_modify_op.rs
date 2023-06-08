use std::sync::atomic::{AtomicI32, Ordering};
fn main() {
    let atomic_a = AtomicI32::new(0);

    // add and return previous value.
    let tmp_1 = atomic_a.fetch_add(1, Ordering::Relaxed);

    // replace and return previous value.
    let tmp_2 = atomic_a.swap(100, Ordering::Relaxed);

    assert_eq!(tmp_1, 0);
    assert_eq!(tmp_2, 1);
    assert_eq!(atomic_a.load(Ordering::Relaxed), 100);
}
