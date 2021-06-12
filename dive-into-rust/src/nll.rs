// #![feature(nll)]
#![allow(dead_code)]

use std::collections::HashMap;

/// Non-Lexical Lifetimes (NLL) has been included in the Rust compiler
/// since 1.31, and has been enabled on the 2015 edition since 1.36.
/// See more in E0729 (`rustc --explain 0729`)
fn process_or_default(map: &mut HashMap<String, String>, key: String) {
    match map.get_mut(&key) {
        Some(value) => println!("{}", value),
        None => {
            map.insert(key, String::new());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_lexical_lifetime() {
        let mut map = HashMap::<String, String>::new();
        let key = String::from("abc");
        process_or_default(&mut map, key);
    }

    #[test]
    fn test_nll() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.push(v.len()); // 同一行既有&借用，也有&mut借用。但逻辑上是安全的。
        println!("{:?}", v);

        // assert!(false);
    }
}
