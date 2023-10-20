// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::{Arc, Mutex};
//use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // let status = Arc::new(JobStatus { jobs_completed: 0 });
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let status_shared = Arc::clone(&status);
    //     let handle = thread::spawn(move || {
    //         thread::sleep(Duration::from_millis(250));
    //         // TODO: You must take an action before you update a shared value
          
    //         status_shared.jobs_completed +=1;
    //        // status_sharedd
           
    //     });
    //     handles.push(handle);
    // }
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 获取互斥锁的锁定，并修改共享值
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
            // 打印共享值的状态
            println!("jobs_completed: {}", status.jobs_completed);
        });
        handles.push(handle);
    }
    for handle in handles {
       //let res= handle.join();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        handle.join().unwrap();
    }
}
