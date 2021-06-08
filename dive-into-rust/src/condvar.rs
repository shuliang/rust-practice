#[cfg(test)]
mod tests {
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_condvar() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();

        thread::spawn(move || {
            println!("child thread sleeping...");
            thread::sleep(Duration::from_secs(2));
            let &(ref lock, ref cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
            println!("child thread {}", *started);
        });
        // wait for the thread to start up
        println!("wait for child thread...");
        let &(ref lock, ref cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        println!("before wait: {}", *started);
        while !*started {
            started = cvar.wait(started).unwrap();
        }
        println!("after wait: {}", *started);
        // assert!(false);
    }
}
