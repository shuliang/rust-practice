#![allow(unused)]

/// 协变：若 `T1 <: T2` 时满足 `C<T1> <: C<T2>`，则C对于参数T是协变关系；
/// 逆变：若 `T1 <: T2` 时满足 `C<T2> <: C<T1>`，则C对于参数T是逆变关系；
/// 不变：上述两种均不成立。
/// 即：如果类型构造器保持了参数的子类型关系，就是协变；
/// 如果逆转了参数的子类型关系，就是逆变。其他情况，就是不变。
/// Rust不支持普通泛型参数类型的协变和逆变，只对生命周期泛型函数存在协变和逆变。
#[cfg(test)]
mod tests {

    /// 协变：`&'static <: &'a`
    #[test]
    fn test_static_ref_covariant() {
        type StrRef<'a> = &'a str;

        fn print_str<'b>(s: StrRef<'b>) {
            println!("{}", s);
        }

        let s: StrRef<'static> = "hello";
        print_str(s);
        assert!(true);
    }

    /// `&'static str <: &'a str` 并且 `&'a &'static str <: &'a &'a str` 成立，
    /// 说明引用类型针对泛型参数T具备协变关系
    #[test]
    fn test_ref_covariant() {
        fn test<'a>(s: &'a &'static str) {
            let local: &'a &'a str = s;
        }
        assert!(true);
    }

    /// `&mut`型指针针对泛型参数T是不变(invariant)的
    #[test]
    fn test_mut_invariant() {
        fn test<'a>(s: &'a mut &'static str) {
            // error[E0308]: mismatched types
            // expected mutable reference `&'a mut &'a str`
            //    found mutable reference `&'a mut &'static str`
            // let local: &'a mut &'a str = s;
        }
        assert!(true);
    }

    /// `Box<&'static str> <: Box<&'a str>`
    /// Box<T>类型针对T参数具备协变关系
    #[test]
    fn test_box_covariant() {
        fn test<'a>(s: Box<&'static str>) {
            let local: Box<&'a str> = s;
        }
        assert!(true);
    }

    /// `fn(&'a str)`类型可以转换为`fn(&'static str)`类型
    /// 而`fn() -> &'a str`类型不能转换为`fn() -> &'static str`类型
    /// 说明`fn(T) -> U`对于泛型参数T具备逆变(原文协变)关系，对于U不具备协变关系
    #[test]
    fn test_fn_argument_type_contravariant() {
        fn test_arg<'a>(f: fn(&'a str)) {
            let local: fn(&'static str) = f;
        }

        fn test_return<'a>(f: fn() -> &'a str) {
            // error[E0308]: mismatched types
            // expected fn pointer `fn() -> &'static str`
            //    found fn pointer `fn() -> &'a str`
            // let local: fn() -> &'static str = f;
        }
        assert!(true);
    }

    /// `fn() -> &'static str`类型可以转换为`fn() -> &'a str`类型
    /// 说明类型`fn(T) -> U`对于参数U具备协变(原文逆变)关系
    #[test]
    fn test_fn_return_type_covariant() {
        fn test_return<'a>(f: fn() -> &'a str) {
            f();
        }

        fn s() -> &'static str {
            return "";
        }
        test_return(s);

        assert!(true);
    }

    #[test]
    fn test_cell_invariant() {
        use std::cell::Cell;
        fn test<'a>(s: Cell<&'static str>) {
            // error[E0308]: mismatched types
            // expected struct `Cell<&'a str>`
            //    found struct `Cell<&'static str>`
            // let local: Cell<&'a str> = s;
        }
        assert!(true);
    }
}
