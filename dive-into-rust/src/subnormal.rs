#[cfg(test)]
mod tests {

    #[test]
    fn test_subnormal() {
        let mut small = f32::EPSILON;
        while small > 0.0 {
            small = small / 2.0;
            println!("{:.064} {:?}", small, small.classify());
        }

        // assert!(false);
    }

    #[test]
    fn test_divided_by_zero() {
        let x = 1.0f32 / 0.0;
        let y = 0.0f32 / 0.0;
        println!("{} {}", x, y); // inf NaN

        // assert!(false)
    }

    #[test]
    fn test_inf() {
        let inf = f32::INFINITY;
        // NaN 0 NaN
        println!("{} {} {}", inf * 0.0, 1.0 / inf, inf / inf);

        // assert!(false);
    }

    #[test]
    fn test_nan_comparsion() {
        let nan = f32::NAN;
        // false false false
        println!("{} {} {}", nan < nan, nan > nan, nan == nan);

        // assert!(false);
    }

    #[test]
    fn test_type_convert() {
        let i = 42;
        let p = &i as *const i32 as *mut i32;
        println!("{:p}", p);

        // assert!(false);
    }
}
