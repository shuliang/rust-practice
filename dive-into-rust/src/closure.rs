#![allow(unused)]

fn closure_forms() {
    let add0 = |a: i32, b: i32| -> i32 {
        return a + b;
    };
    let x = add0(1, 2);
    println!("result: {}", x);

    let add1 = |a, b| -> i32 {
        return a + b;
    };
    let x = add1(1, 2);
    println!("result: {}", x);

    let add2 = |a, b| a + b;
    let x = add2(1, 2);
    println!("result: {}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_forms() {
        closure_forms();

        // assert!(false);
    }
}
