#[cfg(test)]
mod tests {

    #[test]
    fn test_unsafe_var() {
        let x = 1_i32;
        let mut y: u32 = 1;
        let raw_mut = &mut y as *mut u32 as *mut i32 as *mut i64; // it's safe
        unsafe {
            *raw_mut = -1;
        }
        println!("{:X} {:X}", x, y);

        // assert!(false);
    }

    #[test]
    fn test_transmute() {
        let x = vec![1, 2, 3, 4, 5];
        unsafe {
            let t: (usize, usize, usize) = std::mem::transmute_copy(&x);
            println!("{} {} {}", t.0, t.1, t.2);
        }

        // assert!(false);
    }

    // *** swap ***********************************************************
    fn swap<T>(x: &mut T, y: &mut T) {
        unsafe {
            // let mut t: T = std::mem::uninitialized();
            let mut t = std::mem::MaybeUninit::<T>::uninit();
            let t = t.as_mut_ptr();
            std::ptr::copy_nonoverlapping(x, t, 1);
            std::ptr::copy_nonoverlapping(y, x, 1);
            std::ptr::copy_nonoverlapping(t, y, 1);
            std::mem::forget(t);
        }
    }

    #[test]
    fn test_swap() {
        let mut x = 1;
        let mut y = 2;
        swap(&mut x, &mut y);
        // std::mem::swap(&mut x, &mut y);
        println!("{}, {}", x, y);

        // assert!(false);
    }
}
