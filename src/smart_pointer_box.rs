use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // Allow us to access the value using a reference.
    // using '*' it's basically saying to get the value from a reference.
    // when a trait implements Deref, it's the same as using *(t.deref())
    // deref returns a reference because if it doesn't return a reference, the ownership is moved out
    // so using *v it basically saying to use the value that the reference points to.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

pub fn test_box() {
    /*
        Box allows us to store data on the heap instead of stack.
        What is on the stack is the address that points to the heap.
        Used to store types that the size is unknown at compile time.
        Useful when:
            You need to use a value that the size must be known at compile time
            You have a large amount of data data you need to change ownership without copying the value
            You wants the ownership of of something that implements a trait instead of a specific type.

        In this example, We use box to define that List::Cons must have a length of i32 + a usize,
        because Box allow us to store a pointer to something stored on the heap, that's why the compiler
        knows the size when compiling.
    */
    let _list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(1, Box::new(List::Nil))))));

    let x = 5;
    let y = MyBox::new(x);
    let z= Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
}

/*
    In this small example, we use box to wrap a struct that implements a trait Controller,
    The box is a reference to the implementation of a Controller stored on the heap.
*/
pub trait Controller {
    fn press(&self, command: &str) -> ();
}

struct Ps5;

impl Controller for Ps5 {
    fn press(&self, command: &str) -> () {
        println!("button '{:?}' was pressed at the Ps5", command);
    }
}

struct Xbox;

impl Controller for Xbox {
    fn press(&self, command: &str) -> () {
        println!("button '{:?}' was pressed at the Xbox", command);
    }
}

pub enum ConsoleType {
    Ps5,
    Xbox
}

/*
    Here we create a different instance of two structs that implements the same trait.
    They're both being stored on the heap but with different unknown sizes.
*/

fn run(option: &ConsoleType) -> Box<dyn Controller> {
    match option {
        ConsoleType::Ps5 => {
            Box::new(Ps5)
        },
        ConsoleType::Xbox => {
            Box::new(Xbox)
        }
    }
}
pub fn start_console() -> () {
    let ps5 = run(&ConsoleType::Ps5);
    let xbox = run(&ConsoleType::Xbox);

    ps5.press("UP");
    xbox.press("DOW");
    ()
}

