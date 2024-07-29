use std::fmt;
// write a list in Rust
#[allow(dead_code)]
#[derive(Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (List::Cons(lval, lnext), List::Cons(rval, rnext)) => lval == rval && lnext.eq(rnext),
            (List::Nil, List::Nil) => true,
            _ => false,
        }
    }
}

impl<T: PartialEq + fmt::Debug> List<T> {
    pub fn new() -> List<T> {
        List::Nil
    }

    pub fn insert(self, value: T) -> Self {
        List::Cons(value, Box::new(self))
    }

    pub fn delete(self, value: &T) -> Self {
        if let List::Cons(elem, next) = self {
            if elem == *value {
                return *next;
            } else {
                return List::Cons(elem, Box::new(next.delete(value)));
            }
        }
        List::Nil
    }

    pub fn find(&self, value: &T) -> Option<&T> {
        if let List::Cons(val, next) = self {
            if *val == *value {
                return Some(val);
            }
            return next.find(value);
        }
        None
    }

    pub fn update(self, old: &T, new: T) -> Self {
        if let List::Cons(val, next) = self {
            if val == *old {
                return List::Cons(new, next);
            } else {
                return List::Cons(val, Box::new(next.update(old, new)));
            }
        }
        List::Nil
    }

    pub fn print(&self) {
        if let List::Cons(val, next) = self {
            print!("{:?}, ", val);
            next.print();
        } else {
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: List<i32> = List::new();
        assert_eq!(list, List::Nil);
    }

    #[test]
    fn test_insert() {
        let mut list = List::new();
        list = list.insert(1);
        assert_eq!(list, List::Cons(1, Box::new(List::Nil)));
    }

    #[test]
    fn test_delete() {
        let mut list = List::new();
        list = list.insert(1);
        list = list.delete(&1);
        assert_eq!(list, List::Nil);
    }

    #[test]
    fn test_find() {
        let mut list = List::new();
        list = list.insert(1);
        let found = list.find(&1);
        assert_eq!(found, Some(&1));
    }

    #[test]
    fn test_update() {
        let mut list = List::new();
        list = list.insert(1);
        list = list.update(&1, 2);
        assert_eq!(list, List::Cons(2, Box::new(List::Nil)));
    }
}
