use std::thread;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>
}

struct Philosopher {
    name: String,
    left_index: usize,
    right_index: usize
}

impl Philosopher {
    fn new(name: &str, left_index: usize, right_index: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left_index: left_index,
            right_index: right_index
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left_index].lock().unwrap();
        let _right = table.forks[self.right_index].lock().unwrap();

        println!("{} started eating", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(())
        ]
    });

    let philosophers = vec![
        Philosopher::new("Philosopher 1", 0, 1),
        Philosopher::new("Philosopher 2", 1, 2),
        Philosopher::new("Philosopher 3", 2, 3),
        Philosopher::new("Philosopher 4", 3, 4),
        Philosopher::new("Philosopher 5", 0, 4)
    ];

    let thread_handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for handle in thread_handles {
        handle.join().unwrap();
    }
}