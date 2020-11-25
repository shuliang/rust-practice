use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn is_prime(n: usize) -> bool {
    (2..n).all(|i| n % i != 0)
}

fn producer(tx: mpsc::SyncSender<usize>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for i in 100_000_000.. {
            tx.send(i).unwrap();
        }
    })
}

fn worker(id: u64, share_rx: Arc<Mutex<mpsc::Receiver<usize>>>) {
    thread::spawn(move || loop {
        {
            let mut n = 0;
            match share_rx.lock() {
                Ok(rx) => match rx.try_recv() {
                    Ok(_n) => {
                        n = _n;
                    }
                    Err(_) => (),
                },
                Err(_) => (),
            }

            if n != 0 {
                if is_prime(n) {
                    println!("worker {} found a prime: {}", id, n);
                }
            }
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::sync_channel(1024);
    let share_rx = Arc::new(Mutex::new(rx));

    for i in 1..5 {
        worker(i, share_rx.clone());
    }

    producer(tx).join().unwrap();
}
