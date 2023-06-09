use std::{
    sync::atomic::{AtomicI32, AtomicU32, AtomicU64, AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
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
    let total_progress = &AtomicU32::new(0);

    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        for thread_id in 0..4 {
            s.spawn(move || {
                for task_id in 0..25 {
                    let start = Instant::now();
                    thread::sleep(Duration::from_millis(thread_id * 2500 + task_id));
                    let previous = total_progress.fetch_add(1, Ordering::Relaxed);
                    let spend = start.elapsed().as_micros() as u64;
                    total_time.fetch_add(spend, Ordering::Relaxed);
                    max_time.fetch_max(spend, Ordering::Relaxed);
                }
            });
        }

        loop {
            let tmp_progress = total_progress.load(Ordering::Relaxed);
            let total_time = Duration::from_micros(total_time.load(Ordering::Relaxed));
            let max_time = Duration::from_micros(max_time.load(Ordering::Relaxed));
            if tmp_progress == 100 {
                break;
            }
            if tmp_progress > 0 {
                println!(
                    "Working.. {tmp_progress} / 100, avarage: {:?}, max_time: {:?}",
                    total_time / tmp_progress,
                    max_time
                );
            }
            thread::sleep(Duration::from_secs(1))
        }
    });

    println!("Done.");
}
