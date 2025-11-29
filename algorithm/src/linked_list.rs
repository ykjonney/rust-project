// 链表实现集合
// 演示在 Rust 中使用引用计数（Rc）与内部可变性（RefCell）实现链表
// 支持链表基本操作：创建、插入、删除、环检测

use std::cell::RefCell;
use std::fmt::{ Debug};
use std::rc::Rc;

/// 链表节点类型：使用 Option<Rc<RefCell<Node<T>>>> 表示可空的、引用计数的、可变的节点
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// 单链表节点
#[derive(PartialEq, Eq, Clone, Debug)]
struct Node<T:PartialEq> {
    /// 节点数据
    elem: T,
    /// 指向下一个节点的指针
    next: Link<T>,
}

/// 单链表结构
#[derive(Debug)]
struct List<T:PartialEq> {
    /// 头指针
    head: Link<T>,
    /// 尾指针（用于快速追加）
    tail: Link<T>,
}
impl<T:PartialEq+Debug>List<T> {
    /// 创建空链表
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    /// 向链表末尾添加新节点
    fn push(&mut self, new_node: Link<T>) {
        if  self.head.is_none(){
            self.head = new_node.clone();
            self.tail = new_node;
        }else{
            self.tail.as_mut().unwrap().borrow_mut().next = new_node.clone();
            self.tail = new_node;
        }
        
    }

    /// 从链表头部移除并返回节点数据
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

    /// 环检测：使用龟兔赛跑算法检测链表中是否存在环
    /// 若存在环，返回环中的某个节点；否则返回 None
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
