// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn traverse_node(node: Rc<RefCell<TreeNode>>, 
                         current_depth: &mut usize, right_view: &mut Vec<i32>){
        
        let current_node = node.borrow();
        //println!("current node val {} current_depth {}, current_node.val, current_depth, max_depth);

        if *current_depth == right_view.len(){
            right_view.push(current_node.val);    
        }

        match current_node.right.clone(){
            Some(right) => {
                *current_depth += 1;
                Self::traverse_node(right, current_depth, right_view);
            },
            None => {
                //println!("No right leaf");
            }
        }

        match current_node.left.clone(){
            Some(left) => {
                *current_depth += 1;
                Self::traverse_node(left, current_depth, right_view);
            },
            None => {
                //println!("No left leaf");
            }
        }

        *current_depth -= 1;
    }

    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut right_view = Vec::new();
        if root.is_none(){
            return right_view;
        }
        
        let mut current_depth = 0;

        Self::traverse_node(root.unwrap(), &mut current_depth, &mut right_view);
        return right_view;
    }
}