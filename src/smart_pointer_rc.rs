/*
    Rc -> Reference Counted Smart Pointer

    It's a smart pointer that keeps track of how many references to the value,
    to determine whether the value is still in use or can be cleaned up.

     -> Stores data on the heap
     -> Allow multiple owners.
     -> Uses Rc::clone(&rc) to increase the reference counting instead of a deep clone.

     It allow us to store data on the heap tha will be used by different owners,
     the compiler doesn't know which one will be finish using it last.
     It's used only for single thread applications

     The reference to the value is valid as long as there is a owner.
*/

use std::collections::HashMap;
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn test_rc() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    assert_eq!(Rc::strong_count(&a), 1);

    let _b = List::Cons(3, Rc::clone(&a));
    assert_eq!(Rc::strong_count(&a), 2);

    {
        let _c = List::Cons(4, Rc::clone(&a));
        assert_eq!(Rc::strong_count(&a), 3);
    }
    assert_eq!(Rc::strong_count(&a), 2);
}
struct Db {
    data: HashMap<String, i32>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            data: HashMap::<String, i32>::from([
                (String::from("tony stark"), 1),
                (String::from("steve rogers"), 2),
                (String::from("natasha romanoff"), 3),
                (String::from("bruce banner"), 4),
                (String::from("bart clinton"), 5),
                (String::from("thor"), 6),
            ]),
        }
    }

    pub fn get(&self, key: &str) -> i32 {
        (self.data.get(key).map(|c| *c)).unwrap_or_default()
    }
}

fn strongest_avenger(data: Rc<Db>) -> i32 {
    // Once the data goes out of scope, the reference counter decreases by one
    assert_eq!(Rc::strong_count(&data), 3);
    data.get("thor")
}

fn best_avenger(data: Rc<Db>) -> i32 {
    // Once the data goes out of scope, the reference counter decreases by one
    assert_eq!(Rc::strong_count(&data), 3);
    data.get("tony stark")
}

pub fn multiple_dbs() -> () {
    /*
       Here we have a struct called Db that is a expensive data structure to be cloned.
       Instead of cloning the struct, we share the ownership.
    */

    let db = Rc::new(Db::new());
    assert_eq!(Rc::strong_count(&db), 1);

    let _clone_b = Rc::clone(&db);
    assert_eq!(Rc::strong_count(&db), 2);

    let strongest_avenger = strongest_avenger(Rc::clone(&db));
    assert_eq!(Rc::strong_count(&db), 2);

    let best_avenger = best_avenger(Rc::clone(&db));

    assert_eq!(Rc::strong_count(&db), 2);
    assert_eq!(strongest_avenger, 6);
    assert_eq!(best_avenger, 1);
}
