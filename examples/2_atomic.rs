use std::{sync::atomic::AtomicUsize, thread, time::Duration};

fn main() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                thread::sleep(Duration::from_secs(1));

                num_done.store(i + 1, std::sync::atomic::Ordering::Relaxed);
            }
        });

        loop {
            let current_num = num_done.load(std::sync::atomic::Ordering::Relaxed);
            if 100 == current_num {
                break;
            }
            println!("Working...{current_num} / 100 done.");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done.")
}
