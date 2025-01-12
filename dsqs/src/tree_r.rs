// /Maximum Depth of Binary Tree
// use std::cell::RefCell;
// use std::rc::Rc;
// #[derive(Debug, PartialEq, Eq)]

// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

// pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//     match root {
//         None => 0,
//         Some(node) => {
//             let left = max_depth(node.borrow().left.clone());
//             let right = max_depth(node.borrow().right.clone());
//             1 + left.max(right)
//         }
//     }
// }

// Given a binary tree, the task is to find the size of the tree. The size of a tree is the number of nodes present in the tree. 

// Define the structure of a binary tree node
#[derive(Debug)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

// Implement a method to calculate the size of the tree
impl TreeNode {
    pub fn size(&self) -> usize {
        let mut count = 1;

        if let Some(ref left) = self.left {
            count += left.size();
        }

        if let Some(ref right) = self.right {
            count += right.size();
        }

        count
    }
}
