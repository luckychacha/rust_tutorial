use std::{collections::VecDeque, sync::Mutex, thread, time::Duration};

fn main() {
    let queue = Mutex::new(VecDeque::<i32>::new());

    thread::scope(|s| {
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(s) = item {
                dbg!(s);
            } else {
                // park: wait for a notification from another thread
                thread::park();
            }
        });

        for i in 0..100 {
            queue.lock().unwrap().push_back(i);
            // when unpark, the park() call returns and loop continues.
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
