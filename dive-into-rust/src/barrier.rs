#[cfg(test)]
mod tests {
    use std::sync::{Arc, Barrier};

    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(10));
        let mut handlers = vec![];
        for _ in 0..10 {
            let c = barrier.clone();
            let t = std::thread::spawn(move || {
                println!("before wait");
                c.wait();
                println!("after wait");
            });
            handlers.push(t);
        }
        for h in handlers {
            h.join().unwrap();
        }
        // assert!(false);
    }
}
