use add_one;
use rand::prelude::*;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    println!("random: {y}");
}
