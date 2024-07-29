// import lib.rs
use list_rs::List;

fn main() {
    let mut list = List::new();
    for i in 0..100 {
        list = list.insert(i);
    }
    list.print();
    list = list.delete(&50);

    if let Some(val) = list.find(&50) {
        println!("Found 50: {:?}", val);
    } else {
        println!("50 not found");
    }

    if let Some(val) = list.find(&49) {
        println!("Found 49: {:?}", val);
    } else {
        println!("49 not found");
    }
}
