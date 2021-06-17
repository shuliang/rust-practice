#![allow(unused)]

#[cfg(test)]
mod tests {
    // *** trait object *******************************************************

    trait Bird {
        fn fly(&self);
    }

    struct Duck;
    impl Bird for Duck {
        fn fly(&self) {
            println!("duck duck");
        }
    }

    struct Swan;
    impl Bird for Swan {
        fn fly(&self) {
            println!("swan swan");
        }
    }

    fn print_trait_object(p: &dyn Bird) {
        let (data, vtable): (usize, *const usize) = unsafe { std::mem::transmute(p) };
        println!("TraitObject [data:{}, vtable:{:p}]", data, vtable);
        unsafe {
            println!(
                "data in vtable [{}, {}, {}, {}]",
                *vtable,
                *vtable.offset(1),
                *vtable.offset(2),
                *vtable.offset(3)
            );
        }
    }

    #[test]
    fn test_trait_object() {
        let duck = Duck;
        let p_duck = &duck;
        let p_bird = p_duck as &dyn Bird;
        println!(
            "Size of p_duck: {}, Size of p_bird: {}",
            std::mem::size_of_val(&p_duck),
            std::mem::size_of_val(&p_bird)
        );
        let duck_fly: usize = Duck::fly as usize;
        let swan_fly: usize = Swan::fly as usize;
        println!("Duck::fly {}", duck_fly);
        println!("Swan::fly {}", swan_fly);
        print_trait_object(p_bird);
        let swan = Swan;
        print_trait_object(&swan as &dyn Bird);

        // assert!(false);
    }

    // *** object safe ****************************************************

    trait Double {
        // fn new() -> Self; // not work, must `Sized`
        fn new() -> Self
        where
            Self: Sized;
        fn double(&mut self);
    }

    impl Double for i32 {
        fn new() -> Self {
            0
        }

        fn double(&mut self) {
            *self *= 2;
        }
    }

    #[test]
    fn test_object_safe() {
        let mut i = 1;
        let p: &mut dyn Double = &mut i as &mut dyn Double;
        p.double();
    }

    // *** conservative impl trait ****************************************
    fn multiply(m: i32) -> impl Fn(i32) -> i32 {
        move |x| x * m
    }

    fn in_func_params(f: impl Fn(i32) -> i32) {}

    // type MyIter = impl Iterator<Item = i32>;

    // trait MyTrait {}

    // trait InTrait {
    //     fn test() -> impl MyTrait;
    // }

    // trait AssociatedType {
    //     type AT = impl MyTrait;
    //     fn foo() -> Sefl::AT;
    // }

    #[test]
    fn test_conservative_impl_trait() {
        let f = multiply(5);
        println!("{}", f(2));

        // assert!(false);
    }
}
