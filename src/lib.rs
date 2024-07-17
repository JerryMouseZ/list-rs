use std::fmt;
// write a list in Rust
#[allow(dead_code)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: PartialEq + fmt::Debug> List<T> {
    pub fn new() -> List<T> {
        List::Nil
    }

    pub fn insert(self, value: T) -> Self {
        List::Cons(value, Box::new(self))
    }

    pub fn delete(self, value: &T) -> Self {
        match self {
            List::Cons(val, next) => {
                if val == *value {
                    *next
                } else {
                    List::Cons(val, Box::new(next.delete(value)))
                }
            },
            List::Nil => {
                List::Nil
            }
        }
    }

    pub fn find(&self, value: &T) -> &Self {
        let mut node = self;
        while let List::Cons(val, next) = node {
            if *val == *value {
                return &node;
            }
            node = next;
        }
        &List::Nil
    }

    pub fn print(&self) {
        let mut node = self;
        while let List::Cons(val, next) = node {
            print!("{:?}, ", val);
            node = next;
        }
        println!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: List<i32> = List::new();
        match list {
            List::Nil => assert!(true),
            _ => assert!(false, "New list should be Nil"),
        }
    }

    #[test]
    fn test_insert() {
        let mut list = List::new();
        list = list.insert(1);
        match list {
            List::Cons(val, _) => assert_eq!(val, 1),
            _ => assert!(false, "List should not be Nil after insertion"),
        }
    }

    #[test]
    fn test_delete() {
        let mut list = List::new();
        list = list.insert(1);
        list = list.delete(&1);
        match list {
            List::Nil => assert!(true),
            _ => assert!(false, "List should be Nil after deletion"),
        }
    }

    #[test]
    fn test_find() {
        let mut list = List::new();
        list = list.insert(1);
        let found = list.find(&1);
        match found {
            List::Cons(val, _) => assert_eq!(*val, 1),
            _ => assert!(false, "Should find the inserted value"),
        }
    }
}

