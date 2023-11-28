use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}
#[derive(Debug)]
struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}
impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, elem: T) {
        let new_node = Rc::new(RefCell::new(Node { elem, next: None }));
        if self.head.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            self.tail.as_ref().unwrap().borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node);
        }
    }
    fn pop(&mut self) -> Option<T> {
        let node = self.head.take();
        if node.is_none() {
            return None;
        } else {
            if node.as_ref().unwrap().borrow().next.is_none() {
                self.tail = None;
                self.head = None;
                return Some(
                    Rc::try_unwrap(node.unwrap())
                        .ok()
                        .unwrap()
                        .into_inner()
                        .elem,
                );
            } else {
                self.head = node.as_ref().unwrap().borrow().next.clone();
                return Some(
                    Rc::try_unwrap(node.unwrap())
                        .ok()
                        .unwrap()
                        .into_inner()
                        .elem,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.push(6);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
    }
}
