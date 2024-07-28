use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(x: T) -> Self {
        Node::<T> {
            elem: x,
            next: None,
        }
    }
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::<T> {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, x: T) {
        // 生成新节点
        let new_node = Rc::new(RefCell::new(Node::<T>::new(x)));
        // 所有权: list.old_head -> new_node.next
        match self.head.take() {
            // 如果头部为空，头和尾都变成新节点
            None => {
                self.tail = Some(new_node.clone());
            }
            Some(old_head) => {
                new_node.borrow_mut().next = Some(old_head);
            }
        }
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Link<T> {
        // 删除头节点
        // 1. 头节点为空，返回None
        // 2. 头节点为最后一个元素，头和尾都置空
        // 所有权 释放list.head, list.tail
        // 3. 头节点不为最后一个元素
        // 所有权old_head.next -> list.head, 释放list.old_head
        match self.head.take() {
            None => {
                return None;
            }
            Some(head) => {
                match head.borrow_mut().next.take() {
                    None => {
                        self.tail = None;
                        self.head = None;
                    }
                    Some(next) => {
                        self.head = Some(next);
                    }
                }
                Some(head)
            }
        }
    }

    pub fn push_back(&mut self, x: T) {
        let new_node = Rc::new(RefCell::new(Node::new(x)));
        match self.tail.take() {
            None => {
                self.head = Some(new_node.clone());
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
            }
        }
        self.tail = Some(new_node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_front() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front().map(|node| (node).borrow().elem), Some(3));
        assert_eq!(list.pop_front().map(|node| (node).borrow().elem), Some(2));
        assert_eq!(list.pop_front().map(|node| (node).borrow().elem), Some(1));

        assert_eq!(list.pop_front().is_none(), true);
    }

    #[test]
    fn test_push_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_front().map(|node| (node).borrow().elem), Some(1));
        assert_eq!(list.pop_front().map(|node| (node).borrow().elem), Some(2));
        assert_eq!(list.pop_front().map(|node| (node).borrow().elem), Some(3));
    }
}
