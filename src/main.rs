// import lib.rs
use list_rs::List;

fn main() {
    let mut list = List::new();
    for i in 0..100 {
        list = list.insert(i);
    }
    list.print();
    list = list.delete(&50);

    match list.find(&50) {
        List::Cons(val, _) => println!("Found 50: {:?}", val),
        List::Nil => println!("50 not found"),
    }

    match list.find(&49) {
        List::Cons(val, _) => println!("Found 49: {:?}", val),
        List::Nil => println!("49 not found"),
    }
}
