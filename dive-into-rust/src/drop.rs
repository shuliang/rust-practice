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

    // *** drop panic *****************************************************
    // https://github.com/rust-lang/rust/issues/14875
    struct T(u8);

    impl Drop for T {
        fn drop(&mut self) {
            println!("drop {}", self.0);
            if self.0 == 1 {
                panic!()
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_drop_panic() {
        let _v = vec![T(0), T(1), T(2), T(3)];
    }

    // *** drop unwind ****************************************************
    struct HasDrop;
    static mut COUNT: usize = 0;

    impl Drop for HasDrop {
        fn drop(&mut self) {
            unsafe {
                COUNT += 1;
                if COUNT == 5 {
                    panic!()
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_drop_unwind() {
        const N: usize = 10;
        let mut a = Vec::with_capacity(10);
        for _ in 0..N {
            a.push(HasDrop);
        }
        let a_raw = &mut a as *mut _;

        match std::panic::catch_unwind(|| {
            unsafe { std::ptr::drop_in_place(a_raw) };
        }) {
            Err(_) => std::mem::drop(a),
            Ok(()) => std::mem::forget(a),
        }

        unsafe { assert_eq!(COUNT, N) }
    }
}
