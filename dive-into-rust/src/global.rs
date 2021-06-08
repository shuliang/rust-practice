#[cfg(test)]
mod tests {

    static mut G0: i32 = 0;

    #[test]
    fn test_should_use_unsafe_to_change_global_var() {
        unsafe {
            G0 = 1;
        }
    }

    // compile failed
    // use std::{cell::Cell, thread};
    // static G1: Cell<i32> = Cell::new(1);
    // #[test]
    // fn test_global_var_can_not_sync() {
    //     fn f1() {
    //         G1.set(2);
    //     }
    //     fn f2() {
    //         G1.set(3);
    //     }
    //     thread::spawn(|| f1());
    //     thread::spawn(|| f2());
    // }
}
