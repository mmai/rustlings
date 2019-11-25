// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    // let status_shared = Arc::clone(&status);
    let status_shared = Arc::clone(&status);
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut jobStatus = status_shared.lock().unwrap();
            (*jobStatus).jobs_completed += 1;
        };
    });
    let status_shared = Arc::clone(&status);
    let mut jb = 0;
    while jb < 10 {
        thread::sleep(Duration::from_millis(500));
        let jobStatus = status_shared.lock().unwrap();
        jb = jobStatus.jobs_completed;
        println!("waiting... {}", jb);
    }
}
