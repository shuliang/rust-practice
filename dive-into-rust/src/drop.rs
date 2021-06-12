#[cfg(test)]
mod tests {

    #[test]
    fn test_manual_drop() {
        let x = vec![1, 2, 3];
        println!("before drop {:?}", x);
        std::mem::drop(x);
        // error[E0382]: borrow of moved value: `x`
        // println!("after drop {:?}", x);

        // assert!(false);
    }

    struct D(i32);

    impl Drop for D {
        fn drop(&mut self) {
            println!("destructor for {}", self.0);
        }
    }

    #[test]
    fn test_drop_shadowing() {
        let _x = D(1);
        println!("construct first vairable");
        let _x = D(2);
        println!("construct second vairable");

        // assert!(false);
    }

    #[test]
    fn test_drop_underscore() {
        let _x = D(1);
        let _ = D(2);
        let _y = D(3);

        // assert!(false);
    }
}
