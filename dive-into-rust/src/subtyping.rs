#![allow(unused)]

// https://www.youtube.com/watch?v=fI4RG_uq-WU
// http://pnkfx.org/presentations/rustfest-berlin-2016/slides.html

/// (子类型可以隐性的转换为父类型)
/// Int <: Real =>
/// Real -> Int <: Int -> Real (参数逆变, 返回类型协变)

/*

Y <: X
A -> Y <: A -> X

aka -> is covariant with respect to its return type.

Example instance:

Int <: Real
Real -> Int <: Real -> Real
All caller can do is get an X from calling the function;
So it is safe to narrow and use a Y as the return value.




"Have a function of type Real -> Int, want one like Int -> Int"

Sometimes unintuitive

"Client says they will only feed integer values into the function, so it is safe to provide something that can consume any real number."




Y <: X    B <: A
A -> Y <: B -> X

aka -> is covariant with respect to its return type, and -> is contravariant with respect to its argument type.

All caller can do is feed in more specific B (and get out more general X).
So it is safe to be more liberal and accept any A at all, and guarantee the more specific Y as return value.

*/

// *** Cell example *******************************************************

struct MyCell<T> {
    value: T,
}

impl<T: Copy> MyCell<T> {
    fn new(x: T) -> MyCell<T> {
        MyCell { value: x }
    }
    fn get(&self) -> T {
        self.value
    }
    fn set(&self, value: T) {
        use std::ptr;
        unsafe {
            ptr::write(&self.value as *const _ as *mut _, value);
        }
    }
}

static X: i32 = 10;

fn step1<'a>(r_c1: &MyCell<&'a i32>) {
    let val: i32 = 13;
    step2(&val, r_c1);
    println!("step1 value: {}", r_c1.value);
}

fn step2<'b>(r_val: &'b i32, r_c2: &MyCell<&'b i32>) {
    r_c2.set(r_val);
}

#[test]
fn test_mycell_short_lifetime() {
    let cell = MyCell::new(&X);
    step1(&cell);
    println!("  end value: {}", cell.value);

    // assert!(false);
}

/*
mod test_stdcell_short_lifetime {
    use std::cell::Cell;
    static X: i32 = 10;

    #[test]
    fn test_stdcell_short_lifetime() {
        let cell = Cell::new(&X);
        step1(&cell);
    }
    fn step1<'a>(r_c1: &Cell<&'a i32>) {
        let val: i32 = 13;
        // error[E0597]: `val` does not live long enough
        step2(&val, r_c1);
    }
    fn step2<'b>(r_val: &'b i32, r_c2: &Cell<&'b i32>) {
        r_c2.set(r_val);
    }
}


*/

/*

struct MyCell<T> {
    value: T,
}

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

#[lang = "unsafe_cell"]
pub struct UnsafeCell<T: ?Sized> {
    value: T,
}

*/

// *** lifetime ***********************************************************

/// Picks either `x` or `y`, based on some internal choice.
fn pick<'a>(x: &'a i32, y: &'static i32) -> &'a i32 {
    if *x > 0 {
        x
    } else {
        y
    }
}

static GLOBAL: i32 = 100;

#[test]
fn pick_test() {
    let temp: i32 = 200;
    pick(&temp, &GLOBAL);
}

// fn promote<'a>(x: &'a i32) -> &'static i32 {
//     return x;
// }

// #[test]
// fn promote_test() {
//     let temp: i32 = 200;
//     promote(&temp);
// }

/*

Insight: For any type T and any lifetime 'a, clearly &'static T should be valid anywhere that &'a T is.

'static outlives 'a
&'static T <: &'a T



For any type T and lifetimes 'a, 'b, if 'b outlives 'a,
then &'b T should be valid anywhere &'a T is.

'b outlives 'a
&'b T <: &'a T



Already established we should have &'static T <: &'a T.
What about a reference to a reference?
Should &'a &'static T <: &'a &'a T ...?
Intuition: All you can do with a &X is read data from the X it holds.
Analogous to a function A -> Y <: A -> X
'b outlives 'a
Y <: X
&'b Y <: &'a X
&X is covariant with respect to X



But that's &X; what about &mut X? --- Invariance

Insight: for any type X and any lifetime 'a,
&'static mut X is valid anywhere &'a mut X is.
But we cannot generalize to Y <: X

Intuition: Once you allow mutation through a reference,
the type itself must remain fixed.

*/

// *** function type variant **********************************************

mod demo_variance_and_static_ref {
    fn provide(m: &'static i32) {
        let val = 13;
        expect(&val, m);
    }
    fn expect<'a>(_: &'a i32, _r: &'a i32) {
        unimplemented!()
    }
}

mod demo_variance_and_static_ref_hof {
    fn prov_hof(f: fn(&usize) -> &'static i32) {
        let val = 13;
        exp_hof(&val, f);
    }
    fn exp_hof<'a>(_: &'a i32, _f: fn(&'a usize) -> &'a i32) {
        unimplemented!()
    }
}
// Functions even contravariant with respect to their argument types

/*

Where does variance come from?
Compiler deduces the variance of a type (with respect to its type parameters) based on the structure of that type.

struct OuterInline<T> { one: T, inner: InnerInline<T> }
struct InnerInline<T> { data: T }
InnerInline and OuterInline both covariant with respect to T

struct OuterRef<'a, T: 'a> { one: &'a mut T, inner: InnerRef<'a, T> }
struct InnerRef<'a, T: 'a> { data: &'a T }
InnerRef is covariant w.r.t. T, while OuterRef is invariant w.r.t. T

If compiler sees a PhantomData<SomeType>, it traverses the structure of SomeType as if it were embedded directly.



What's up with MyCell
Structural definition of MyCell alone implies it is covariant w.r.t. T

This (broken) method violates rules associated with covariance:
```rust
    fn set(&self, value: T) {
        use std::ptr;
        unsafe {
            ptr::write(&self.value as *const _ as *mut _, value);
        }
    }
```
Must impose invariance

Use PhantomData<fn (T) -> T> in MyCell<T>: one way

Use UnsafeCell<T> in MyCell<T>: better way



Pop Quiz
&'static mut T <: &'a mut T
&'a &'static mut T <: &'a &'a mut T
&'a mut &'static T <: &'a mut &'a T
In case you care: Yes, Yes, No

*/
