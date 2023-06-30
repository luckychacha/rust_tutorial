use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 有界 channel
    // bounded channel: the [`Receiver`] will block until a message becomes available.
    // let (tx, rx) = mpsc::sync_channel(3);
    // 无界 channel: infinite buffer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: got {msg}");
    }
}
