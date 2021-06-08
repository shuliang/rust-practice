#![allow(dead_code)]

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Self {
        Self {
            name: name.to_string(),
            left,
            right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        println!("{} take left fork: {}.", self.name, self.right);
        thread::sleep(Duration::from_secs(1));
        // let _right = table.forks[self.right].lock().unwrap();
        println!("{} take right fork: {}.", self.name, self.right);
        thread::sleep(Duration::from_secs(1));
        println!("{} is finished eating", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_deadlock() {
        let table = Arc::new(Table {
            forks: vec![
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
            ],
        });

        let philosophers = vec![
            Philosopher::new("A", 0, 1),
            Philosopher::new("B", 1, 2),
            Philosopher::new("C", 2, 3),
            Philosopher::new("D", 3, 4),
            Philosopher::new("E", 4, 0),
        ];

        let handles: Vec<_> = philosophers
            .into_iter()
            .map(|p| {
                let table = table.clone();
                thread::spawn(move || {
                    p.eat(&table);
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        println!("All done.");
        // assert!(false);
    }
}
