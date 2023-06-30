use std::{sync::Arc, thread};

fn main() {
    let a = Arc::new(vec![1, 2, 3]);
    // let b = a.clone();

    thread::spawn({
        let a = a.clone();
        move || dbg!(a)
    })
    .join()
    .unwrap();
    // thread::spawn(move || dbg!(a)).join().unwrap();
    dbg!(a);
}
