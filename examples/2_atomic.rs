use std::{sync::atomic::AtomicUsize, thread, time::Duration};

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                thread::sleep(Duration::from_micros(i * 1000));
                // thread::sleep(Duration::from_secs(1));

                num_done.store(i as usize + 1, std::sync::atomic::Ordering::Relaxed);
                main_thread.unpark();
            }
        });

        loop {
            let current_num = num_done.load(std::sync::atomic::Ordering::Relaxed);
            if 100 == current_num {
                break;
            }
            println!("Working...{current_num} / 100 done.");
            // thread::sleep(Duration::from_secs(1));
            // thread will be blocked during `park_timeout`, and will be awake when unpark or each duration reached.
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done.")
}
