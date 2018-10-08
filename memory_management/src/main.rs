use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let map = Box::new(2048);
    println!("{}", map);
    let ptr = Box::into_raw(map);
    println!("{:?}", ptr);

    print_chars(5, "X");
}

fn print_chars(n: usize, c: &str) {
    for i in 0..n {
        let s = Rc::new(Cell::new(i + 2));
        unsafe {
            for j in 0..i + 1 {
                (*s).set(c.as_ptr() as usize);
            }

            println!("{:?}", s);
        }
    }
}
