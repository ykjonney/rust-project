
#[derive(Debug,Clone)]
struct TreeNode<T:Ord> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    // 使用 Option<Box<..>> 来表示可空的堆上递归结构
    right: Option<Box<TreeNode<T>>>,
    height: i32,
}

impl <T:Ord>TreeNode<T> {
    fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
            height: 0,
        }
    }
}

struct AVLTree<T:Ord> {
    root: Option<Box<TreeNode<T>>>,
} 
impl <T:Ord + Clone>AVLTree<T> {
    fn new() -> Self {
        AVLTree { root: None }
    }

    fn insert(&mut self,val:T){
        let node = self.root.take();
        self.root = Self::insert_helper(node,val);
        
    }
    fn insert_helper(node:Option<Box<TreeNode<T>>>,val:T)->Option<Box<TreeNode<T>>>{
        if node.is_none(){
            return Some(Box::new(TreeNode::new(val)));
        }
        let mut n = node.unwrap();
        if val < n.val{
            n.left = Self::insert_helper(n.left,val);
        }else if val > n.val{
            n.right = Self::insert_helper(n.right,val);
        }else{
            // 相等不插入
            return Some(n);
        }
        Self::update_height(&mut n);
        Self::rebalance(n)
    }

    // 删除节点
    fn remove(&mut self,val:T){
        let node = self.root.take();
        self.root = Self::remove_helper(node,val);
    }
    fn remove_helper(node:Option<Box<TreeNode<T>>>,val:T)->Option<Box<TreeNode<T>>>{
        if node.is_none(){
            return None;
        }
        let mut n = node.unwrap();
        if val < n.val{
            n.left = Self::remove_helper(n.left.take(),val);
        }else if val > n.val{
            n.right = Self::remove_helper(n.right.take(),val);
        }else{
            // 找到节点
            if n.left.is_none(){
                return n.right;
            }else if n.right.is_none(){
                return n.left;
            } else {
                // 有两个子节点，找到右子树的最小节点替换
                let mut right_min = n.right.take().unwrap();
                let mut parent = &mut right_min;
                while let Some(ref mut left_child) = parent.left {
                    parent = left_child;
                }
                // 复制右子树最小节点的值，替换当前节点的值
                let min_val = parent.val.clone();
                n.val = min_val.clone();
                // 从右子树中删除该最小值节点并把结果赋回 n.right
                n.right = Self::remove_helper(Some(right_min), min_val);
            }
        }
        Self::update_height(&mut n);
        Self::rebalance(n)

    }
    // 获取节点高度
    fn get_height(node:&Option<Box<TreeNode<T>>>) -> i32 {
        if let Some(n) = node {
            n.height
        } else {
            -1
        }
    }
    // 更新节点高度
    fn update_height(node:&mut Box<TreeNode<T>>){
        let left_height = Self::get_height(&node.left);
        let right_height = Self::get_height(&node.right);
        node.height = 1 + left_height.max(right_height);
    }
    // 获取平衡因子
    fn get_balance_factor(node:&Box<TreeNode<T>>) -> i32 {
        Self::get_height(&node.left) - Self::get_height(&node.right)
    }
    // 右旋
    fn right_rotate(mut node:Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        let mut child = node.left.take().expect("Left child should not be None for right rotation");
        let grandchild = child.right.take();
        // 进行旋转
        node.left = grandchild;
       Self::update_height(&mut node);
       child.right = Some(node);
       Self::update_height(&mut child);

       child
    }

    // 左旋
    fn left_rotate(mut node:Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        let mut child = node.right.take().expect("Right child should not be None for left rotation");
        let grandchild = child.left.take();
        // 进行旋转
        node.right = grandchild;
        Self::update_height(&mut node);
        child.left = Some(node);
        Self::update_height(&mut child);

        child
    }

    fn rebalance(mut node:Box<TreeNode<T>>) ->Option<Box<TreeNode<T>>> {
        let balance_factor = Self::get_balance_factor(&node);
        // 左重
        if balance_factor > 1 {
            // 左-右情况
            if Self::get_balance_factor(node.left.as_ref().unwrap()) < 0 {
                let left_child = node.left.take().unwrap();
                node.left = Some(Self::left_rotate(left_child));
            }
            // 左-左情况
            return Some(Self::right_rotate(node));
        }
        // 右重
        if balance_factor < -1 {
            // 右-左情况
            if Self::get_balance_factor(&node.right.as_ref().unwrap()) > 0 {
                let right_child = node.right.take().unwrap();
                node.right = Some(Self::right_rotate(right_child));
            }
            // 右-右情况
            return Some(Self::left_rotate(node));
        }
        Some(node)
    }
}  