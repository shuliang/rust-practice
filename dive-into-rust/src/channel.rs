#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn test_channel_sp() {
        let (tx, rx) = std::sync::mpsc::channel();
        thread::spawn(move || {
            for i in 0..10 {
                tx.send(i).unwrap();
            }
        });

        while let Ok(r) = rx.recv() {
            println!("received: {}", r);
        }

        // assert!(false);
    }

    #[test]
    fn test_channel_mp() {
        let (tx, rx) = std::sync::mpsc::channel();
        for i in 0..10 {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(i).unwrap();
            });
        }
        drop(tx);

        while let Ok(r) = rx.recv() {
            println!("received: {}", r);
        }

        // assert!(false);
    }

    #[test]
    fn test_sync_channel() {
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        tx.send(1).unwrap();
        println!("send first");
        thread::spawn(move || {
            tx.send(2).unwrap();
            println!("send second");
        });

        println!("receive first {}", rx.recv().unwrap());
        println!("receive second {}", rx.recv().unwrap());

        // assert!(false);
    }
}
