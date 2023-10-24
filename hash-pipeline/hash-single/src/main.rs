// [[file:../../hash-pipeline.org::hash-single.rs][hash-single.rs]]
use sha2::{Digest, Sha512};
use std::time::Instant;

const N: usize = 1_000_000_000;

fn main() {
    println!("hash-single start...");
    let start = Instant::now();
    for i in 0..N {
        let preimage = (i as u64).to_le_bytes();
        Sha512::digest(preimage);
        blake3::hash(&preimage);
    }
    println!("total time: {:?}", start.elapsed());
}
// hash-single.rs ends here
