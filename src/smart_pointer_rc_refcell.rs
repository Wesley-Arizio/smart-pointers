/*
   RefCell and Rc

   Basically lets you mutate data even when there is a immutable reference,
   (you can have either BUT not both at the same time).
   It uses unsafe code wrapped in a Safe API to bend the ownership and mutability rules,
   instead of checking them at compile time, it does at runtime, if you use the api wrongly,
   the application will panic.

   The main difference from Box<T> and RefCell is when the borrowing rules are applied
   Box<T>      -> enforce the rules at compile time.
   RefCell<T>  -> enforce the rules at run time.
   RefCell allow you to mutate data, Box doesn't.

   RefCell itself is immutable but the value inside is mutable, that's called Interior Mutability Pattern
*/

use std::cell::RefCell;
use std::rc::Rc;

pub fn test_rc_refcell() {
    /*
        In this example, we have a counter that has multiple owners using
        Rc, through RefCell we're able to update the value on all instances,
        because they are the same reference.
    */

    let rc_v = Rc::new(RefCell::new(0));
    assert_eq!(Rc::strong_count(&rc_v), 1);

    let instance1 = Rc::clone(&rc_v);
    assert_eq!(Rc::strong_count(&rc_v), 2);

    let instance2 = Rc::clone(&rc_v);
    assert_eq!(Rc::strong_count(&rc_v), 3);

    assert_eq!(0, *instance1.borrow());
    assert_eq!(0, *instance2.borrow());

    let mut m = rc_v.borrow_mut();

    *m += 1;

    // If we don't drop this mutable reference, we cannot borrow other mutable or immutable reference
    drop(m);
    assert_eq!(1, *instance1.borrow());
    assert_eq!(1, *instance2.borrow());

    *rc_v.borrow_mut() += 1;

    assert_eq!(2, *instance1.borrow());
    assert_eq!(2, *instance2.borrow());
}
