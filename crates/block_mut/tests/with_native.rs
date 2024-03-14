use std::{cell::Cell, rc::Rc};

use block_mut::{Block, BoxBlock};

#[link(name = "block_muttest", kind = "dylib")]
extern "C" {
    fn check_addition<'f>(a: i32, b: i32, cb: &Block<dyn FnMut(i32, i32) -> i32 + 'f>) -> bool;
    fn check_addition2(a: i32, b: i32, cb: &Block<dyn FnMut(i32, i32) -> i32>) -> bool;
}

#[test]
fn test_check_addition() {
    assert!(unsafe { check_addition(1, 2, &BoxBlock::new(|a, b| a + b)) });
    assert!(unsafe { !check_addition(1, 2, &BoxBlock::new(|a, b| a + b + 1)) });
}

#[test]
fn test_mut() {
    let mut ret: i32 = 0;
    let cb = |a, b| {
        ret = a + b;
        ret
    };
    assert!(unsafe { check_addition(1, 2, &BoxBlock::new(cb)) });
    assert_eq!(ret, 3);
}

#[test]
fn test_mut_cell() {
    let ret0 = Rc::new(Cell::new(0));
    let ret = ret0.clone();
    let cb = move |a, b| {
        ret.set(a + b);
        ret.get()
    };
    assert!(unsafe { check_addition2(1, 2, &BoxBlock::new(cb)) });
    assert_eq!(ret0.get(), 3);
}
