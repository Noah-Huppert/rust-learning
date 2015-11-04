extern crate time;

use std::thread;

#[no_mangle]
pub extern fn process() {
    let start_time = time::precise_time_s();

    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut x = 0;

            for _ in (0..999_999_999) {
                x += 1
            }

            x
        })
    }).collect();

    for h in handles {
        let result = h.join().map_err(|_| "Could not join a thread!").unwrap();

        println!("Thread finished with count {}", result);
    }

    println!("Done in {}", time::precise_time_s() - start_time);
}