#![allow(dead_code)]

#[cfg(test)]
mod tests {

    struct T {
        memeber: i32,
    }

    fn foo<'a>(arg: &'a T) -> &'a i32 {
        &arg.memeber
    }

    fn bar<'a, 'b>(arg: &'a T) -> &'b i32
    where
        'a: 'b,
    {
        &arg.memeber
    }

    #[test]
    fn test_lifetime_outlives() {
        let t = T { memeber: 1 };
        let x = foo(&t);
        println!("{:?}", x);
        let x = bar(&t);
        println!("{:?}", x);

        // assert!(false);
    }

    // *** elision ******************************************************

    fn get_str_0(s: &String) -> &str {
        s.as_ref()
    }

    fn get_str_1<'a>(s: &'a String) -> &'a str {
        s.as_ref()
    }

    fn get_str_static_0(s: &String) -> &'static str {
        println!("call fn {}", s);
        "hello world"
    }

    fn get_str_static_1<'a>(s: &'a String) -> &'static str {
        println!("call fn {}", s);
        "hello world"
    }

    #[test]
    fn test_lifetime_elision() {
        let c = "haha".to_string();

        let x: &str = get_str_0(&c);
        println!("{}", x);

        let x: &str = get_str_1(&c);
        println!("{}", x);

        let x: &'static str = get_str_static_0(&c);
        println!("{}", x);

        // assert!(false);
    }
}
