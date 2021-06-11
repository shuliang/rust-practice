#[cfg(test)]
mod tests {
    // *** impl trait for trait *******************************************
    trait Shape {
        fn area(&self) -> f64;
    }

    trait Round {
        fn get_radius(&self) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    impl Round for Circle {
        fn get_radius(&self) -> f64 {
            self.radius
        }
    }

    // Note: impl Trait for Trait
    impl Shape for dyn Round {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.get_radius() * self.get_radius()
        }
    }

    #[test]
    fn test_impl_trait_for_trait() {
        let _c = Circle { radius: 2.0 };
        // _c.area(); // compile error
        let b = Box::new(Circle { radius: 4.0 }) as Box<dyn Round>;
        println!("{}", b.area());

        // assert!(false);
    }

    // *** Fully Qualified Syntax (Universal Funtion Call Syntax) *********
    // `<T as TraitName>::item`
    trait Cook {
        fn start(&self);
    }

    trait Wash {
        fn start(&self);
    }

    struct Chef;

    impl Cook for Chef {
        fn start(&self) {
            println!("Cook::start");
        }
    }

    impl Wash for Chef {
        fn start(&self) {
            println!("Wash::start");
        }
    }

    #[test]
    fn test_ufcs() {
        let me = Chef;
        // me.start(); // error[E0034]: multiple applicable items in scope

        <dyn Cook>::start(&me);
        <dyn Wash>::start(&me);

        <Chef as Cook>::start(&me);
        <Chef as Wash>::start(&me);

        // assert!(false);
    }

    // *** 成员变量与普通方法本质相同 *************************************
    struct T(usize);
    impl T {
        fn get1(&self) -> usize {
            self.0
        }
        fn get2(&self) -> usize {
            self.0
        }
    }
    fn get3(t: &T) -> usize {
        t.0
    }
    fn check_type(_: fn(&T) -> usize) {}

    #[test]
    fn test_trait_function() {
        check_type(T::get1);
        check_type(T::get2);
        check_type(get3);

        // assert!(false);
    }
}
