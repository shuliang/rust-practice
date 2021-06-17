#[cfg(test)]
mod tests {

    use std::ops::{Generator, GeneratorState};
    use std::pin::Pin;

    #[test]
    fn test_generator_seq() {
        let mut generator = || {
            println!("2");
            yield;
            println!("4");
        };

        println!("1");
        Pin::new(&mut generator).resume(());
        println!("3");
        Pin::new(&mut generator).resume(());
        println!("5");

        // assert!(false);
    }

    #[test]
    fn test_generator_fibonacci() {
        let mut g = || {
            let mut curr: u64 = 1;
            let mut next: u64 = 1;
            loop {
                let new_next = curr.checked_add(next);
                if let Some(new_next) = new_next {
                    if new_next > 1000 {
                        return;
                    }
                    curr = next;
                    next = new_next;
                    yield curr;
                } else {
                    return;
                }
            }
        };

        for _ in 0..1000 {
            match Pin::new(&mut g).resume(()) {
                GeneratorState::Yielded(v) => println!("{}", v),
                GeneratorState::Complete(_) => return,
            }
        }

        // assert!(false);
    }
}
