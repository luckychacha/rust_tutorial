use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
    time::Duration,
};

#[allow(dead_code)]
fn basic_op() {
    let atomic_a = AtomicI32::new(0);

    // add and return previous value.
    let tmp_1 = atomic_a.fetch_add(1, Ordering::Relaxed);

    // replace and return previous value.
    let tmp_2 = atomic_a.swap(100, Ordering::Relaxed);

    assert_eq!(tmp_1, 0);
    assert_eq!(tmp_2, 1);
    assert_eq!(atomic_a.load(Ordering::Relaxed), 100);
}

fn main() {
    let total_progress = &AtomicI32::new(0);

    thread::scope(|s| {
        for thread_id in 0..4 {
            s.spawn(move || {
                for task_id in 0..25 {
                    thread::sleep(Duration::from_millis(thread_id * 2500 + task_id));
                    let previous = total_progress.fetch_add(1, Ordering::Relaxed);
                    println!(
                        "{:?}: previous: {previous} after: {}",
                        thread::current().id(),
                        previous + 1
                    );
                }
            });
        }

        loop {
            let tmp_progress = total_progress.load(Ordering::Relaxed);
            if tmp_progress == 100 {
                break;
            }
            println!("Working.. {tmp_progress} / 100");
            thread::sleep(Duration::from_secs(1))
        }
    });

    println!("Done.");
}
