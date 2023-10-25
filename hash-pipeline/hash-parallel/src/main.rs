// [[file:../../hash-pipeline.org::hash-parallel.rs][hash-parallel.rs]]
use rtrb::{Consumer, Producer};
use sha2::{Digest, Sha512};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
use std::time::Instant;

const N: usize = 1_000_000_000;
const CAPACITY: usize = 1_000_000;

const NUM_SHA512_HASHERS: usize = 2;
const NUM_BLAKE3_HASHERS: usize = 2;

fn main() {
    println!("hash-parallel start...");
    let start = Instant::now();
    let (mut generator_to_sha512_tx, mut generator_to_sha512_rx) =
        ring_buffer(NUM_SHA512_HASHERS, CAPACITY);
    let (mut generator_to_blake3_tx, mut generator_to_blake3_rx) =
        ring_buffer(NUM_BLAKE3_HASHERS, CAPACITY);
    let (mut sha512_to_result_tx, mut sha512_to_result_rx) =
        ring_buffer(NUM_SHA512_HASHERS, CAPACITY);
    let (mut blake3_to_result_tx, mut blake3_to_result_rx) =
        ring_buffer(NUM_BLAKE3_HASHERS, CAPACITY);

    // Generator
    thread::spawn(move || {
        let mut sha512_channel = 0;
        let mut blake3_channel = 0;
        for i in 0..N {
            let preimage = (i as u64).to_le_bytes();
            push(&mut generator_to_sha512_tx[sha512_channel], preimage);
            push(&mut generator_to_blake3_tx[blake3_channel], preimage);
            sha512_channel = (sha512_channel + 1) % NUM_SHA512_HASHERS;
            blake3_channel = (blake3_channel + 1) % NUM_BLAKE3_HASHERS;
        }
    });

    // Sha512
    for _ in 0..NUM_SHA512_HASHERS {
        let mut rx = generator_to_sha512_rx.pop_front().unwrap();
        let mut tx = sha512_to_result_tx.pop_front().unwrap();
        thread::spawn(move || loop {
            let preimage = pop(&mut rx);
            let hash = Sha512::digest(preimage);
            push(&mut tx, hash);
        });
    }

    // Blake3
    for _ in 0..NUM_BLAKE3_HASHERS {
        let mut rx = generator_to_blake3_rx.pop_front().unwrap();
        let mut tx = blake3_to_result_tx.pop_front().unwrap();
        thread::spawn(move || loop {
            let preimage = pop(&mut rx);
            let hash = blake3::hash(&preimage);
            push(&mut tx, hash);
        });
    }

    // Result
    let result_thread = thread::spawn(move || {
        let mut sha512_channel = 0;
        let mut blake3_channel = 0;
        for _ in 0..N {
            pop(&mut sha512_to_result_rx[sha512_channel]);
            pop(&mut blake3_to_result_rx[blake3_channel]);
            sha512_channel = (sha512_channel + 1) % NUM_SHA512_HASHERS;
            blake3_channel = (blake3_channel + 1) % NUM_BLAKE3_HASHERS;
        }
    });

    result_thread.join().unwrap();

    println!("total time: {:?}", start.elapsed());
}

fn ring_buffer<T>(count: usize, capacity: usize) -> (VecDeque<Producer<T>>, VecDeque<Consumer<T>>) {
    (0..count).map(|_| rtrb::RingBuffer::new(capacity)).unzip()
}

fn push<T>(tx: &mut Producer<T>, mut value: T) {
    loop {
        match tx.push(value) {
            Ok(_) => break,
            Err(rtrb::PushError::Full(v)) => value = v,
        }
        thread::sleep(Duration::from_millis(1));
    }
}

fn pop<T>(rx: &mut Consumer<T>) -> T {
    loop {
        if let Ok(value) = rx.pop() {
            return value;
        }
        thread::sleep(Duration::from_millis(1));
    }
}
// hash-parallel.rs ends here
