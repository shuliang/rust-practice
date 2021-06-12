#![allow(dead_code)]

#[cfg(test)]
mod tests {

    #[test]
    fn test_auto_deref() {
        let s = "hello";
        println!("lenght: {}", s.len());
        println!("lenght: {}", (&s).len());
        println!("lenght: {}", (&&&&&&&&&&&&s).len());
    }

    #[test]
    fn test_rc_auto_deref() {
        use std::ops::Deref;
        use std::rc::Rc;

        let s = Rc::new(String::from("hello"));
        println!("lenght: {}", s.len());
        println!("lenght: {}", s.deref().len());
        println!("lenght: {}", s.deref().deref().len());
        println!("lenght: {}", (*s).len());
        println!("lenght: {}", (&*s).len());
        println!("lenght: {}", (&**s).len());

        // assert!(false);
    }

    fn joint() {
        let s = Box::new(String::from("joint"));
        let p = &*s;
        println!("joint: {:?} {:?}", p, s);
    }

    fn seperate() {
        let s = Box::new(String::from("seperate"));
        let tmp = *s; // moved s
        let p = &tmp;
        println!("seperate: {:?}", p);
        // println!("seperate: {:?} {:?}", p, s); // compile failure
    }

    #[test]
    fn test_ref_deref_joint_seperate() {
        joint();
        seperate();

        // assert!(false);
    }

    use std::borrow::Cow;
    fn remove_space<'a>(input: &'a str) -> Cow<'a, str> {
        if input.contains(' ') {
            let mut buf = String::with_capacity(input.len());
            for c in input.chars() {
                if c != ' ' {
                    buf.push(c);
                }
            }
            return Cow::Owned(buf);
        }
        return Cow::Borrowed(input);
    }

    #[test]
    fn test_cow() {
        let s1 = "no_spaces_in_string";
        let result1 = remove_space(s1);
        let s2 = "spaces in string";
        let result2 = remove_space(s2);
        println!("{}\n{}", result1, result2);

        // assert!(false);
    }
}
