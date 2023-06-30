// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: Philosopher
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork(usize);

struct Philosopher {
    name: String,
    // ANCHOR_END: Philosopher
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>,
}

// ANCHOR: Philosopher-think
impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }
    // ANCHOR_END: Philosopher-think

    // ANCHOR: Philosopher-eat
    fn eat(&self) {
        let id = thread::current().id();
        // ANCHOR_END: Philosopher-eat
        // println!("id: {:?}: {} is trying to eat", id, &self.name);
        // // println!("{} waiting left", &self.name);
        // println!("id: {:?}: {} wait left fork", id, &self.name);
        // let left = self.left_fork.lock().unwrap();
        // println!("id: {:?}: {} got left fork: {}", id, &self.name, left.0);
        // // println!("{} got left and waiting right", &self.name);
        // println!("id: {:?}: {} wait right fork", id, &self.name);
        // let right = self.right_fork.lock().unwrap();
        // println!("id: {:?}: {} got right fork: {}", id, &self.name, right.0);
        // println!("id: {:?}: {} wait left fork", id, &self.name);

        println!("id: {:?}: {} wait left fork", id, &self.name);
        if let Ok(_) = self.left_fork.try_lock() {
            println!("id: {:?}: {} wait right fork", id, &self.name);
            if let Ok(_) = self.right_fork.try_lock() {
                // ANCHOR: Philosopher-eat-end
                println!("{} is eating...", &self.name);
                thread::sleep(Duration::from_millis(10));
            } else {
                println!("id: {:?}: {} failed to get right fork", id, &self.name);
            }
        } else {
            println!("id: {:?}: {} failed to get left fork", id, &self.name);
        }
    }
}

static PHILOSOPHERS: &[&str] = &[
    "0-Socrates",
    "1-Plato",
    "2-Aristotle",
    "3-Thales",
    "4-Pythagoras",
];

fn main() {
    // ANCHOR_END: Philosopher-eat-end
    let (tx, rx) = mpsc::sync_channel(10);

    let forks = (0..PHILOSOPHERS.len())
        .map(|i| Arc::new(Mutex::new(Fork(i))))
        .collect::<Vec<_>>();

    for i in 0..forks.len() {
        let tx = tx.clone();
        let mut left_fork = forks[i].clone();
        let mut right_fork = forks[(i + 1) % forks.len()].clone();

        // To avoid a deadlock, we have to break the symmetry
        // somewhere. This will swap the forks without deinitializing
        // either of them.
        // if i == forks.len() - 1 {
        //     std::mem::swap(&mut left_fork, &mut right_fork);
        // }

        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx,
            left_fork,
            right_fork,
        };

        thread::spawn(move || {
            for _ in 0..100 {
                philosopher.eat();
                philosopher.think();
            }
        });
    }

    drop(tx);
    for thought in rx {
        println!("{thought}");
    }
}
