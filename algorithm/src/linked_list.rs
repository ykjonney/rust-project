use std::cell::RefCell;
use std::fmt::{ Debug};
use std::rc::Rc;


type Link<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(PartialEq, Eq, Clone, Debug)]
struct Node<T:PartialEq> {
    elem: T,
    next: Link<T>,
}
#[derive(Debug)]
struct List<T:PartialEq> {
    head: Link<T>,
    tail: Link<T>,
}
impl<T:PartialEq+Debug>List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, new_node: Link<T>) {
        if  self.head.is_none(){
            self.head = new_node.clone();
            self.tail = new_node;
        }else{
            self.tail.as_mut().unwrap().borrow_mut().next = new_node.clone();
            self.tail = new_node;
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
      // 判断链表是否有环，如果有返回节点
    fn has_cycle(&self) -> Option<Rc<RefCell<Node<T>>>>{
        let  mut fast = self.head.clone();
        let  mut slow = self.head.clone();
        while fast.is_some() && fast.as_ref().unwrap().borrow().next.is_some(){
            slow = slow.unwrap().borrow().next.clone();
            fast = fast.unwrap().borrow().next.as_ref().unwrap().borrow().next.clone();
            if Rc::ptr_eq(slow.as_ref().unwrap(), fast.as_ref().unwrap()){
                let mut head = self.head.clone();
                while !Rc::ptr_eq(head.as_ref().unwrap(), fast.as_ref().unwrap()){
                    head = head.unwrap().borrow().next.clone();
                    fast = fast.unwrap().borrow().next.clone();
                    
                }
                return head;
            }
            
        }
        None
      }
}

  


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list() {
        let mut list = List::new();
        list.push(Some(Rc::new(RefCell::new(Node { elem: 1, next: None }))));
        list.push(Some(Rc::new(RefCell::new(Node { elem: 2, next: None }))));
        let node3 = Some(Rc::new(RefCell::new(Node { elem: 3, next: None })));
        list.push(node3);
        list.push(Some(Rc::new(RefCell::new(Node { elem: 4, next: None }))));
        list.push(Some(Rc::new(RefCell::new(Node { elem: 5, next: None }))));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
    }
    #[test]
    fn test_linked_list_cycle() {
        let mut list = List::new();
        let node1 = Some(Rc::new(RefCell::new(Node { elem: 1, next: None })));
        let node2 = Some(Rc::new(RefCell::new(Node { elem: 2, next: None })));
        let node3 = Some(Rc::new(RefCell::new(Node { elem: 3, next: None })));
        let node4 = Some(Rc::new(RefCell::new(Node { elem: 4, next: None })));
        list.push(node1.clone());
        list.push(node2.clone());
        list.push(node3.clone());
        list.push(node4.clone());
        list.push(node2.clone());
        let head = list.has_cycle();
        println!("{:?}",head.as_ref().unwrap().borrow().elem);
        assert_eq!(head,node2);
    }
}
