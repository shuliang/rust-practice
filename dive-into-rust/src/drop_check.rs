#![allow(unused)]

#[cfg(test)]
mod tests {

    /// Elements' lifetime in tuple and slice are "equal"
    #[test]
    fn test_drop_check_equal_lifetime() {
        {
            let (a, mut b): (i32, Option<&i32>) = (1, None);
            b = Some(&a);
        }
        {
            let (mut a, b): (Option<&i32>, i32) = (None, 1);
            a = Some(&b);
        }

        // assert!(false);
    }

    // *** may_dangle *****************************************************
    struct T {
        dropped: bool,
    }
    impl T {
        fn new() -> Self {
            Self { dropped: false }
        }
    }

    struct R<'a> {
        inner: Option<&'a T>,
    }
    impl<'a> R<'a> {
        fn new() -> Self {
            Self { inner: None }
        }
        fn set_ref<'b: 'a>(&mut self, ptr: &'b T) {
            self.inner = Some(ptr);
        }
    }
    // error[E0658]: `may_dangle` has unstable semantics and may be removed in the future
    // help: add `#![feature(dropck_eyepatch)]` to the crate attributes to enable
    unsafe impl<#[may_dangle] 'a> Drop for R<'a> {
        fn drop(&mut self) {
            if let Some(ref inner) = self.inner {
                println!("droppen R when T is {}", inner.dropped);
            }
        }
    }

    #[test]
    fn test_drop_check_may_dangle() {
        {
            let (a, mut b): (T, R) = (T::new(), R::new());
            b.set_ref(&a);
        }
        {
            let (mut a, b): (R, T) = (R::new(), T::new());
            // without`may_dangle`, fails: error[E0597]: `b` does not live long enough
            a.set_ref(&b);
        }
    }
}
