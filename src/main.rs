/*
    Smart pointers
    They store a reference to some data stored anywhere (heap or stack).
    Smart pointers own the data they point to.
    Box -> allocating values on the heap
    Rc -> a reference counting type that enables multiple ownership
    Ref and RefMut -> Type that reinforces borrowing rules at runtime and is
    accessed through RefCell
*/

mod smart_pointer_box;
mod smart_pointer_rc;

use smart_pointer_box::{start_console, test_box};
use smart_pointer_rc::{multiple_dbs, test_rc};

fn main() {
    test_box();
    start_console();
    test_rc();
    multiple_dbs()
}
