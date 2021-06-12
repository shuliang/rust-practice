// interior mutability - inherited mutability

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_cell_mut() {
        let data: Cell<i32> = Cell::new(100);
        let p = &data;
        data.set(10);
        println!("{}", p.get());
        p.set(20);
        println!("{:?}", data);

        // assert!(false);
    }

    #[test]
    fn test_rc() {
        let r1 = Rc::new(1);
        println!("reference count: {}", Rc::strong_count(&r1));
        let r2 = r1.clone();
        println!("reference count: {}", Rc::strong_count(&r2));

        // assert!(false);
    }

    #[test]
    fn test_refcell_mut_compile() {
        let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec![1, 2, 3]);
        let shared1 = &shared_vec;
        let shared2 = &shared1;
        shared1.borrow_mut().push(4);
        println!("{:?}", shared_vec.borrow()); // borrow() return `Ref<T>`
        println!("{:?}", shared_vec);
        println!("{:?}", shared1);
        println!("{:?}", shared2);
        println!();
        shared2.borrow_mut().push(5);
        println!("{:?}", shared_vec.borrow()); // borrow() return `Ref<T>`
        println!("{:?}", shared_vec);
        println!("{:?}", shared1);
        println!("{:?}", shared2);
        println!("{:?}", shared2.clone());

        // assert!(false);
    }

    #[test]
    #[should_panic]
    fn test_refcell_mut_runtime_verify() {
        let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec![1, 2, 3]);
        let shared1 = &shared_vec;
        let shared2 = &shared1;
        let p1 = shared1.borrow();
        let p2 = &p1[0];
        // panicked at 'already borrowed: BorrowMutError', src/interior_mut.rs:60:17
        shared2.borrow_mut().push(4);
        println!("{}", p2);
    }
}
