use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    thread::scope(|s| {
        // Add 50000 to the counter in a background thread
        let t1 = s.spawn(|| {
            for _ in 0..50000 {
                *counter.lock().unwrap() += 1;
            }
        });

        // Add 50000 to the counter in a background thread
        let t2 = s.spawn(|| {
            for _ in 0..50000 {
                *counter.lock().unwrap() += 1;
            }
        });

        // Wait for both threads to finish
        t1.join().unwrap();
        t2.join().unwrap();

        println!("Result = {:?}", *counter.lock().unwrap());

        // todo: make it better.
    });
}
