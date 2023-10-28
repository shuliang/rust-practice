// [[file:../../hash-pipeline.org::hash-idle-blocked.rs][hash-idle-blocked.rs]]
#![allow(non_snake_case)]
use rtrb::{Consumer, Producer};
use sha2::{Digest, Sha512};
use std::collections::VecDeque;
use std::env;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

const N: usize = 1_000_000_000;
const CAPACITY: usize = 1_000_000;

// const NUM_SHA512_HASHERS: usize = 2;
// const NUM_BLAKE3_HASHERS: usize = 2;

fn main() {
    // println!("hash-idle-blocked start...");
    let mut NUM_SHA512_HASHERS: usize = 2;
    if let Some(arg) = env::args().nth(1) {
        NUM_SHA512_HASHERS = arg.parse().unwrap();
    }
    let mut NUM_BLAKE3_HASHERS: usize = 2;
    if let Some(arg) = env::args().nth(2) {
        NUM_BLAKE3_HASHERS = arg.parse().unwrap();
    }
    let start = Instant::now();
    let (mut generator_to_sha512_tx, mut generator_to_sha512_rx) =
        ring_buffer(NUM_SHA512_HASHERS, CAPACITY);
    let (mut generator_to_blake3_tx, mut generator_to_blake3_rx) =
        ring_buffer(NUM_BLAKE3_HASHERS, CAPACITY);
    let (mut sha512_to_result_tx, mut sha512_to_result_rx) =
        ring_buffer(NUM_SHA512_HASHERS, CAPACITY);
    let (mut blake3_to_result_tx, mut blake3_to_result_rx) =
        ring_buffer(NUM_BLAKE3_HASHERS, CAPACITY);

    let mut stats = vec![];

    // Generator
    let (idle, blocked) = (Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0)));
    stats.push(("generator".to_string(), idle.clone(), blocked.clone()));
    thread::spawn(move || {
        let mut sha512_channel = 0;
        let mut blake3_channel = 0;
        for i in 0..N {
            let preimage = (i as u64).to_le_bytes();
            push(
                &mut generator_to_sha512_tx[sha512_channel],
                preimage,
                &blocked,
            );
            push(
                &mut generator_to_blake3_tx[blake3_channel],
                preimage,
                &blocked,
            );
            sha512_channel = (sha512_channel + 1) % NUM_SHA512_HASHERS;
            blake3_channel = (blake3_channel + 1) % NUM_BLAKE3_HASHERS;
        }
    });

    // Sha512
    for i in 0..NUM_SHA512_HASHERS {
        let (idle, blocked) = (Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0)));
        stats.push((format!("sha512_{:_>2}", i), idle.clone(), blocked.clone()));
        let mut rx = generator_to_sha512_rx.pop_front().unwrap();
        let mut tx = sha512_to_result_tx.pop_front().unwrap();
        thread::spawn(move || loop {
            let preimage = pop(&mut rx, &idle);
            let hash = Sha512::digest(preimage);
            push(&mut tx, hash, &blocked);
        });
    }

    // Blake3
    for i in 0..NUM_BLAKE3_HASHERS {
        let (idle, blocked) = (Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0)));
        stats.push((format!("blake3_{:_>2}", i), idle.clone(), blocked.clone()));
        let mut rx = generator_to_blake3_rx.pop_front().unwrap();
        let mut tx = blake3_to_result_tx.pop_front().unwrap();
        thread::spawn(move || loop {
            let preimage = pop(&mut rx, &idle);
            let hash = blake3::hash(&preimage);
            push(&mut tx, hash, &blocked);
        });
    }

    // Result
    let (idle, blocked) = (Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0)));
    stats.push(("result   ".to_string(), idle.clone(), blocked.clone()));
    let result_thread = thread::spawn(move || {
        let mut sha512_channel = 0;
        let mut blake3_channel = 0;
        for _ in 0..N {
            pop(&mut sha512_to_result_rx[sha512_channel], &idle);
            pop(&mut blake3_to_result_rx[blake3_channel], &idle);
            sha512_channel = (sha512_channel + 1) % NUM_SHA512_HASHERS;
            blake3_channel = (blake3_channel + 1) % NUM_BLAKE3_HASHERS;
        }
    });

    // Stats
    thread::spawn(move || {
        let start = Instant::now();
        loop {
            for (name, idle, blocked) in stats.iter() {
                let percent_idle = (100.0 * idle.load(Ordering::Relaxed) as f64
                    / start.elapsed().as_millis() as f64) as i32;
                let percent_blocked = (100.0 * blocked.load(Ordering::Relaxed) as f64
                    / start.elapsed().as_millis() as f64)
                    as i32;
                println!(
                    "{}: %idle={:>2} %blocked={:>2}",
                    name, percent_idle, percent_blocked
                );
            }
            println!();
            thread::sleep(Duration::from_secs(1));
        }
    });

    result_thread.join().unwrap();

    println!(
        "sha512 = {:>2}, blake3 = {:>2}, time = {:?}",
        NUM_SHA512_HASHERS,
        NUM_BLAKE3_HASHERS,
        start.elapsed()
    );
}

fn ring_buffer<T>(count: usize, capacity: usize) -> (VecDeque<Producer<T>>, VecDeque<Consumer<T>>) {
    (0..count).map(|_| rtrb::RingBuffer::new(capacity)).unzip()
}

fn push<T>(tx: &mut Producer<T>, mut value: T, blocked: &Arc<AtomicU64>) {
    loop {
        match tx.push(value) {
            Ok(_) => break,
            Err(rtrb::PushError::Full(v)) => value = v,
        }
        let start = Instant::now();
        thread::sleep(Duration::from_millis(10));
        blocked.fetch_add(start.elapsed().as_millis() as u64, Ordering::Relaxed);
    }
}

fn pop<T>(rx: &mut Consumer<T>, idle: &Arc<AtomicU64>) -> T {
    loop {
        if let Ok(value) = rx.pop() {
            return value;
        }
        let start = Instant::now();
        thread::sleep(Duration::from_millis(10));
        idle.fetch_add(start.elapsed().as_millis() as u64, Ordering::Relaxed);
    }
}
// hash-idle-blocked.rs ends here
