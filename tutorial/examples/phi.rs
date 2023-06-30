use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建一个共享的 Mutex
    let mutex = Arc::new(Mutex::new(0));

    // 创建多个线程来同时获取锁
    let threads: Vec<_> = (0..500)
        .map(|i| {
            let mutex = Arc::clone(&mutex);
            thread::spawn(move || {
                let result = std::panic::catch_unwind(|| {
                    // 尝试获取锁
                    let lock_result = mutex.lock();

                    // 处理锁被 "poisoned" 的情况
                    match lock_result {
                        Ok(mut guard) => {
                            // 获取到了锁，执行一些操作
                            *guard += 1;
                            println!("Thread: {:?} - Value: {}", thread::current().id(), *guard);
                        }
                        Err(poisoned) => {
                            // 锁被 "poisoned"，进行处理
                            println!(
                                "Thread: {:?} - Lock poisoned, recovering...",
                                thread::current().id()
                            );
                            let guard = poisoned.into_inner();
                            println!("Recovered value: {}", *guard);

                            // 将错误信息写入文件
                            let mut file =
                                File::create("error.log").expect("Failed to create error.log");
                            writeln!(file, "Recovered value: {}", *guard)
                                .expect("Failed to write to error.log");
                        }
                    }
                });

                // 检查是否捕获到panic
                if let Err(panic) = result {
                    println!(
                        "Thread: {:?} - Panic occurred: {:?}",
                        thread::current().id(),
                        panic
                    );
                }

                // 引入 panic，模拟锁被 "poisoned"
                if i == 2 {
                    panic!("Simulated panic");
                }
            })
        })
        .collect();

    // 捕获子线程的panic
    for thread in threads {
        let _ = thread.join().map_err(|e| {
            if let Some(payload) = e.downcast_ref::<&str>() {
                // 子线程发生panic时的处理逻辑
                println!("Child thread panicked: {}", payload);
            } else {
                // 其他类型的panic处理逻辑
                println!("Child thread panicked");
            }
        });
    }
}
