#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn test_primitive_size() {
        // Some primitives
        assert_eq!(0, size_of::<()>());
        assert_eq!(1, size_of::<i8>());
        assert_eq!(4, size_of::<i32>());
        assert_eq!(8, size_of::<i64>());
        assert_eq!(16, size_of::<i128>());
        assert_eq!(8, size_of::<isize>());
        assert_eq!(8, size_of::<usize>());
        assert_eq!(4, size_of::<f32>());
        assert_eq!(8, size_of::<f64>());
        assert_eq!(4, size_of::<char>());
        assert_eq!(4, size_of::<Option<char>>());
        assert_eq!(2, size_of::<Option<i8>>());
        assert_eq!(8, size_of::<Option<i32>>());
        assert_eq!(8, size_of::<Box<i32>>());
        assert_eq!(8, size_of::<Option<Box<i32>>>());
        // pointer
        assert_eq!(8, size_of::<&()>());
        assert_eq!(8, size_of::<&i8>());
        assert_eq!(8, size_of::<&i32>());
        assert_eq!(8, size_of::<&i64>());
        assert_eq!(8, size_of::<&f32>());
        assert_eq!(8, size_of::<&f64>());
        assert_eq!(8, size_of::<&char>());
        assert_eq!(8, size_of::<&Option<char>>());
        assert_eq!(8, size_of::<&Option<i32>>());
        assert_eq!(16, size_of::<&str>()); // fat pointer
        assert_eq!(8, size_of::<&Option<&str>>());
        assert_eq!(8, size_of::<*const i8>());
        assert_eq!(8, size_of::<*mut i8>());
        assert_eq!(8, size_of::<*const i128>());
        assert_eq!(8, size_of::<*mut i128>());
        // Pointer size equality
        assert_eq!(size_of::<&i32>(), size_of::<*const i32>());
        assert_eq!(size_of::<&i32>(), size_of::<*mut i32>());
        assert_eq!(size_of::<&i32>(), size_of::<Box<i32>>());
        assert_eq!(size_of::<&i32>(), size_of::<Option<&i32>>());
        assert_eq!(size_of::<Box<i32>>(), size_of::<Option<Box<i32>>>());
        // Some arrays
        assert_eq!(0, size_of::<[(); 2]>());
        assert_eq!(16, size_of::<[&(); 2]>());
        assert_eq!(2, size_of::<[i8; 2]>());
        assert_eq!(8, size_of::<[i32; 2]>());
        assert_eq!(12, size_of::<[i32; 3]>());
        assert_eq!(0, size_of::<[i32; 0]>());
        assert_eq!(8, size_of::<[f32; 2]>());
        assert_eq!(12, size_of::<[f32; 3]>());
        assert_eq!(0, size_of::<[f32; 0]>());
        // slice
        let psize = size_of::<usize>();
        assert_eq!(psize, size_of::<&[i8; 2]>());
        assert_eq!(16, size_of::<&[i8]>()); // fat pointer
        assert_eq!(16, size_of::<&mut [i8]>()); // fat pointer
        assert_eq!(8, size_of::<&[i32; 2]>());
        assert_eq!(8, size_of::<&[f64; 3]>());
        assert_eq!(8, size_of::<&[i32; 0]>());
        assert_eq!(16, size_of::<&[i32]>()); // fat pointer
        assert_eq!(16, size_of::<&mut [i32]>()); // fat pointer

        let word_len = size_of::<usize>();
        assert_eq!(3 * word_len, size_of::<Vec<u8>>());
        assert_eq!(3 * word_len, size_of::<Vec<char>>());
        assert_eq!(3 * word_len, size_of::<Vec<isize>>());
        assert_eq!(3 * word_len, size_of::<Vec<i128>>());
    }

    // *** Struct **************************************************************

    #[test]
    fn test_empty_struct_size() {
        struct Foo1;
        assert_eq!(0, size_of::<Foo1>());

        #[repr(C)]
        struct Bar1;
        assert_eq!(0, size_of::<Bar1>());

        struct Foo2();
        assert_eq!(0, size_of::<Foo2>());

        struct Foo3 {}
        assert_eq!(0, size_of::<Foo3>());

        struct Foo4 {
            x: i8,
        }
        assert_eq!(1, size_of::<Foo4>());
    }

    #[test]
    fn test_struct_seq_size() {
        struct Foo1 {
            x: i8,  // 1
            y: i32, // 4
            z: f64, // 8
        }
        assert_eq!(16, size_of::<Foo1>());

        struct Foo2 {
            x: i8,  // 1
            y: f64, // 8
            z: i32, // 4
            b: i8,  // 1
        }
        assert_eq!(16, size_of::<Foo2>());

        #[repr(C)]
        struct Bar1 {
            x: i8,  // 1
            y: i32, // 4
            z: f64, // 8
        }
        assert_eq!(16, size_of::<Bar1>());

        #[repr(C)]
        struct Bar2 {
            x: i8,  // 1
            y: f64, // 8
            z: i32, // 4
        }
        assert_eq!(24, size_of::<Bar2>());
    }

    #[test]
    fn test_struct_seq_size_2() {
        struct Foo1 {
            x: i8,   // 1
            a: char, // 4
            y: i32,  // 4
            z: f64,  // 8
        }
        assert_eq!(24, size_of::<Foo1>());

        struct Foo2 {
            x: i8,   // 1
            y: i32,  // 4
            z: f64,  // 8
            a: char, // 4
        }
        assert_eq!(24, size_of::<Foo2>());

        struct Foo3 {
            x: i8,   // 1
            y: f64,  // 8
            z: i32,  // 4
            a: char, // 4
        }
        assert_eq!(24, size_of::<Foo3>());

        #[repr(C)]
        struct Bar1 {
            x: i8,   // 1
            a: char, // 4
            y: i32,  // 4
            z: f64,  // 8
        }
        assert_eq!(24, size_of::<Bar1>());

        #[repr(C)]
        struct Bar2 {
            x: i8,   // 1
            y: i32,  // 4
            z: f64,  // 8
            a: char, // 4
        }
        assert_eq!(24, size_of::<Bar2>());

        #[repr(C)]
        struct Bar3 {
            x: i8,   // 1
            y: f64,  // 8
            z: i32,  // 4
            a: char, // 4
        }
        assert_eq!(24, size_of::<Bar3>());
    }

    #[test]
    fn test_tuple_struct() {
        struct Color1(i8, i8, i8);
        assert_eq!(3, size_of::<Color1>());

        #[repr(C)]
        struct Color2(i8, i8, i8);
        assert_eq!(3, size_of::<Color2>());

        struct Color3(i8, i8, i32); // 1, 1, 4
        assert_eq!(8, size_of::<Color3>());

        #[repr(C)]
        struct Color4(i8, i8, i32); // 1, 1, 4
        assert_eq!(8, size_of::<Color4>());

        struct Color5(i8, i32, i8); // 1, 4, 1
        assert_eq!(8, size_of::<Color5>());

        #[repr(C)]
        struct Color6(i8, i32, i8); // 1, 4, 1
        assert_eq!(12, size_of::<Color6>());

        struct Color7(i32, i8, i8); // 4, 1, 1
        assert_eq!(8, size_of::<Color7>());

        #[repr(C)]
        struct Color8(i32, i8, i8); // 4, 1, 1
        assert_eq!(8, size_of::<Color8>());
    }

    // *** Enum **************************************************************

    #[test]
    fn test_enum_size() {
        enum Foo1 {}
        assert_eq!(0, size_of::<Foo1>());

        // can't compile
        // error[E0084]: unsupported representation for zero-variant enum
        // #[repr(C)]
        // enum Bar1 {};
        // assert_eq!(0, size_of::<Bar1>());

        enum Foo2 {
            A(i8),  // 1
            B(i32), // 4
            C(f64), // 8
        }
        assert_eq!(16, size_of::<Foo2>());

        #[repr(C)]
        enum Bar2 {
            A(i8),  // 1
            B(i32), // 4
            C(f64), // 8
        }
        assert_eq!(16, size_of::<Bar2>());

        enum Foo3 {
            A(i8),  // 1
            C(f64), // 8
            B(i32), // 4
        }
        assert_eq!(16, size_of::<Foo3>());

        #[repr(C)]
        enum Bar3 {
            A(i8),  // 1
            C(f64), // 8
            B(i32), // 4
        }
        assert_eq!(16, size_of::<Bar3>());

        enum Foo4 {
            C(f64), // 8
            A(i8),  // 1
            B(i32), // 4
        }
        assert_eq!(16, size_of::<Foo4>());

        #[repr(C)]
        enum Bar4 {
            C(f64), // 8
            A(i8),  // 1
            B(i32), // 4
        }
        assert_eq!(16, size_of::<Bar4>());

        enum Foo5 {
            A(i8),   // 1
            B(i32),  // 4
            C(f64),  // 8
            D(i128), // 16
        }
        assert_eq!(24, size_of::<Foo5>());

        #[repr(C)]
        enum Bar5 {
            A(i8),   // 1
            D(i128), // 16
            B(i32),  // 4
            C(f64),  // 8
        }
        assert_eq!(24, size_of::<Bar5>());
    }

    // *** Union **************************************************************

    #[test]
    fn test_union_size() {
        // can't compile
        // error: unions cannot have zero fields
        // union Foo1 {};
        // assert_eq!(0, size_of::<Foo1>());

        union Foo2 {
            x: i8,
            y: i32,
            z: i128,
        }
        assert_eq!(16, size_of::<Foo2>());

        #[repr(C)]
        union Bar2 {
            x: i8,
            y: i32,
            z: i128,
        }
        assert_eq!(16, size_of::<Bar2>());

        union Foo3 {
            x: i8,
            z: i128,
            y: i32,
        }
        assert_eq!(16, size_of::<Foo3>());

        #[repr(C)]
        union Bar3 {
            x: i8,
            z: i128,
            y: i32,
        }
        assert_eq!(16, size_of::<Bar3>());
    }

    // *** Slice *************************************************************
    fn raw_slice(arr: &[i32]) {
        unsafe {
            let (v1, v2): (usize, usize) = std::mem::transmute(arr);
            println!("Value in raw pointer:");
            println!("v1: {:x}", v1);
            println!("v2: {:x}", v2);
        }
    }
    #[test]
    fn test_raw_slice() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let address: &[i32; 5] = &arr;
        println!("Address of arr: {:p}", address);
        raw_slice(address);
        raw_slice(address as &[i32]);

        // assert!(false);
    }
}
