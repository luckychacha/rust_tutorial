use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let c = counter.clone();
    {
        let counter_guard = c.lock().unwrap();
    }
    thread::scope(|s| {
        // Add 50000 to the counter in a background thread
        let t1 = s.spawn(|| {
            let mut i = 0;
            for _ in 0..50000 {
                i += 1
            }
            *counter.lock().unwrap() += i;
        });

        // Add 50000 to the counter in a background thread
        let t2 = s.spawn(|| {
            let mut i = 0;
            for _ in 0..50000 {
                i += 1
            }
            *counter.lock().unwrap() += i;
        });

        // Wait for both threads to finish
        t1.join().unwrap();
        t2.join().unwrap();

        println!("Result = {:?}", *counter.lock().unwrap());
    });

    {
        let counter_guard = c.lock().unwrap();
        println!("Result = {:?}", *counter_guard);
    }
}
