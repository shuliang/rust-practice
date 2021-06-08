#[cfg(test)]
mod tests {

    #[test]
    fn test_parking() {
        use parking_lot::Mutex;
        use std::sync::{mpsc::channel, Arc};
        use std::thread;

        const N: usize = 10;
        let data = Arc::new(Mutex::new(0));

        let (tx, rx) = channel();
        for _ in 0..10 {
            let (data, tx) = (Arc::clone(&data), tx.clone());
            thread::spawn(move || {
                let mut data = data.lock();
                *data += 1;
                if *data == N {
                    tx.send(*data).unwrap();
                }
            });
        }
        drop(tx);

        println!("received0: {}", rx.recv().unwrap());
        while let Ok(r) = rx.recv() {
            println!("received1: {}", r);
        }

        // assert!(false);
        assert_eq!(N, *data.lock());
    }

    #[test]
    fn test_std_mutex() {
        use std::sync::Mutex;
        use std::sync::{mpsc::channel, Arc};
        use std::thread;

        const N: usize = 10;
        let data = Arc::new(Mutex::new(0));

        let (tx, rx) = channel();
        for _ in 0..10 {
            let (data, tx) = (Arc::clone(&data), tx.clone());
            thread::spawn(move || {
                let mut data = data.lock().unwrap();
                *data += 1;
                if *data == N {
                    tx.send(*data).unwrap();
                }
            });
        }
        drop(tx);

        println!("received0: {}", rx.recv().unwrap());
        while let Ok(r) = rx.recv() {
            println!("received1: {}", r);
        }

        // assert!(false);
        assert_eq!(N, *data.lock().unwrap());
    }
}
