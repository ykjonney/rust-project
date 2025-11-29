// 二叉搜索树（bst）

use std::{cmp::Ordering, cell::RefCell, rc::Rc};

struct TreeNode {
    value: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

struct BST {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        }))
    }
}

impl BST {
    fn new() -> Self {
        BST { root: None }
    }

    fn insert(&mut self, value: i32) {
        let new_node = TreeNode::new(value);
        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }

        let mut cur = self.root.clone();
        let mut pre = None;
        while let Some(node) = cur.clone(){
            match value.cmp(&node.borrow().value){
                Ordering::Less=>{
                    pre = cur.clone();
                    cur = node.borrow().left.clone();
                },
                Ordering::Greater=>{
                    pre = cur.clone();
                    cur = node.borrow().right.clone();
                },
                Ordering::Equal=>{
                    return;
                }
            }
        }
        // 插入节点
        if let Some(parent) = pre{
            if parent.borrow().value > value{
                parent.borrow_mut().left = Some(new_node);
            }else{
                parent.borrow_mut().right = Some(new_node);
            }
        }
        
    }

    fn search(&self,value:i32) -> Option<Rc<RefCell<TreeNode>>>{
        let mut cur = self.root.clone();
        while let Some(node) = cur.clone(){
            match value.cmp(&node.borrow().value){
                Ordering::Less=>{
                    cur = node.borrow().left.clone();
                },
                Ordering::Greater=>{
                    cur = node.borrow().right.clone();
                },
                Ordering::Equal=>break,
            }
        }
        cur
    }

   fn remove(&mut self,value:i32){
        // 若树为空，直接返回
        if self.root.is_none(){
            return;
        }
        let mut cur = self.root.clone();
        let mut pre = None;
        //循环查找，越过叶节点跳出
        while let Some(node) = cur.clone() {
            match value.cmp(&node.borrow().value){
                Ordering::Less=>{
                    pre = cur.clone();
                    cur = node.borrow().left.clone();
                },
                Ordering::Greater=>{
                    pre = cur.clone();
                    cur = node.borrow().right.clone();
                },
                Ordering::Equal=>{
                    break;  
            }
        }}
        //若无带删除节点，直接返回
        if cur.is_none(){
            return;
        }
        let cur = cur.unwrap();
        let (left_child,right_child) = (cur.borrow().left.clone(),cur.borrow().right.clone());
        match (left_child.clone(),right_child.clone()){
            // 子节点数量 = 0 or 1
            (None,None)|(Some(_),None)|(None,Some(_)) =>{
                // 当子节点数量=0/1时，child = nullptr/唯一子节点
                let child = left_child.or(right_child);
                let pre = pre.unwrap();
                // 
                if !Rc::ptr_eq(&cur, self.root.as_ref().unwrap()){
                    let left = pre.borrow().left.clone();
                    if left.is_some() && Rc::ptr_eq(left.as_ref().unwrap(), &cur){
                        pre.borrow_mut().left = child;
                    } else {
                        pre.borrow_mut().right = child;
                    }
                }else{
                    self.root = child;
                }

            },
            (Some(_),Some(_))=>{
                // 子节点数量 = 2
                // 获取中序遍历中cur的下一个节点
                let mut tmp = cur.borrow().right.clone();
                while let Some(node) = tmp.clone() {
                    if node.borrow().left.is_some(){
                        tmp = node.borrow().left.clone();
                    }else{
                        break;
                    }
                }
                let tmp_val = tmp.unwrap().borrow().value;
                // 递归删除该节点
                self.remove(tmp_val);
                // 用该节点值替换cur节点值
                cur.borrow_mut().value = tmp_val;
            }
        }
    }}



// 二叉搜索平衡树（avl）



// 红黑树




