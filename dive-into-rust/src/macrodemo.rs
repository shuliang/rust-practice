macro_rules! hashmap {
	($($key: expr => $val: expr), *) => {{
		let mut map = std::collections::HashMap::new();
        $(map.insert($key, $val);) *
        map
	}}
}

/// run `rustc +nightly -Z unstable-options --pretty=expanded macrodemo.rs`
fn _t() {
    let counts = hashmap!['A'=>0, 'C'=>0,'G'=>0, 'T'=>0];
    println!("{:?}", counts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_demo() {
        _t();

        // let counts = hashmap!['A'=>0, 'C'=>0,'G'=>0, 'T'=>0];
        // println!("{:?}", counts);

        // assert!(false);
    }
}
