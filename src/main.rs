// import lib.rs
use list_rs::List;

fn main() {
    let mut list = List::new();
    list.push_front(1);
    list.push_front(2);
    while let Some(node) = list.pop_front() {
        println!("{}", (*node).borrow().elem);
    }
}
