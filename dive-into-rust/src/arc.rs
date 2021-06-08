#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
    use std::sync::{Arc, Mutex, RwLock};
    use std::thread;

    #[test]
    fn test_arc() {
        let numbers: Vec<_> = (0..10u32).collect();
        let shared_numbers = Arc::new(numbers);
        for _ in 0..10 {
            let child_numbers = shared_numbers.clone();
            thread::spawn(move || {
                let local_numbers = &child_numbers[..];
                println!("local_numbers: {:?}", local_numbers);
            });
        }
        assert!(true);
    }

    #[test]
    fn test_arc_mutex() {
        const COUNT: u32 = 1000000;
        let global = Arc::new(Mutex::new(0));
        let clone1 = global.clone();
        let thread1 = thread::spawn(move || {
            for _ in 0..COUNT {
                let mut value = clone1.lock().unwrap();
                *value += 1;
            }
        });

        let clone2 = global.clone();
        let thread2 = thread::spawn(move || {
            for _ in 0..COUNT {
                let mut value = clone2.lock().unwrap();
                *value -= 1;
            }
        });
        thread1.join().ok();
        thread2.join().ok();
        println!("final value: {:?}", global);
        assert_eq!(0, *global.clone().lock().unwrap());
    }

    #[test]
    fn test_arc_rwlock() {
        const COUNT: u32 = 1000000;
        let global = Arc::new(RwLock::new(0));
        let clone1 = global.clone();
        let thread1 = thread::spawn(move || {
            for _ in 0..COUNT {
                let mut value = clone1.write().unwrap();
                *value += 1;
            }
        });

        let clone2 = global.clone();
        let thread2 = thread::spawn(move || {
            for _ in 0..COUNT {
                let mut value = clone2.write().unwrap();
                *value -= 1;
            }
        });
        thread1.join().ok();
        thread2.join().ok();
        println!("final value: {:?}", global);
        assert_eq!(0, *global.clone().read().unwrap());
    }

    #[test]
    fn test_arc_atomic() {
        const COUNT: u32 = 1000000;
        let global = Arc::new(AtomicIsize::new(0));
        let clone1 = global.clone();
        let thread1 = thread::spawn(move || {
            for _ in 0..COUNT {
                clone1.fetch_add(1, Ordering::SeqCst);
            }
        });

        let clone2 = global.clone();
        let thread2 = thread::spawn(move || {
            for _ in 0..COUNT {
                clone2.fetch_sub(1, Ordering::SeqCst);
            }
        });
        thread1.join().ok();
        thread2.join().ok();
        println!("final value: {:?}", global);
        assert_eq!(0, global.clone().load(Ordering::Acquire));
    }

    #[test]
    fn test_arc_atomic_without_fetch() {
        const COUNT: u32 = 1000000;
        let global = Arc::new(AtomicIsize::new(0));
        let clone1 = global.clone();
        let thread1 = thread::spawn(move || {
            for _ in 0..COUNT {
                let mut value = clone1.load(Ordering::SeqCst);
                value += 1;
                clone1.store(value, Ordering::SeqCst);
            }
        });

        let clone2 = global.clone();
        let thread2 = thread::spawn(move || {
            for _ in 0..COUNT {
                let mut value = clone2.load(Ordering::SeqCst);
                value -= 1;
                clone2.store(value, Ordering::SeqCst);
            }
        });
        thread1.join().ok();
        thread2.join().ok();
        println!("final value: {:?}", global);
        assert_ne!(0, global.clone().load(Ordering::SeqCst));
    }

    #[test]
    fn test_ordering() {
        let x0 = Arc::new(AtomicBool::new(false));
        let y0 = Arc::new(AtomicBool::new(false));
        let z0 = Arc::new(AtomicIsize::new(0));
        let x1 = x0.clone();
        let y1 = y0.clone();
        let t1 = thread::spawn(move || {
            x1.store(true, Ordering::Relaxed);
            y1.store(true, Ordering::Relaxed);
        });

        let x2 = x0.clone();
        let y2 = y0.clone();
        let z2 = z0.clone();
        let t2 = thread::spawn(move || {
            while !y2.load(Ordering::Relaxed) {}
            if x2.load(Ordering::Relaxed) {
                z2.fetch_add(1, Ordering::SeqCst);
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();

        assert_ne!(z0.clone().load(Ordering::SeqCst), 0);
        assert_eq!(z0.clone().load(Ordering::SeqCst), 1);
    }
}
