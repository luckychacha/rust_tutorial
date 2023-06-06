use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
    thread,
    time::Duration,
};

// use condvar to notify one or all.
// a condvar is only used with one single Mutex.

fn main() {
    let queue = Mutex::new(VecDeque::<i32>::new());
    let empty_wait = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    q = empty_wait.wait(q).unwrap();
                }
            };
            drop(q);
            // dbg!(std::time::Instant::now());
            dbg!(item);
        });

        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    q = empty_wait.wait(q).unwrap();
                }
            };
            drop(q);
            // dbg!(std::time::Instant::now());
            dbg!(item);
        });

        for i in 0..100 {
            queue.lock().unwrap().push_back(i);
            // when unpark, the park() call returns and loop continues.
            // t.thread().unpark();

            // empty_wait.notify_one();
            empty_wait.notify_all();
            // thread::sleep(Duration::from_secs(1));
        }
    });
}
