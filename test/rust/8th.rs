// 8. Given a binary tree, implement a function that returns the maximum depth of the tree.



use std::cmp;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => 1 + cmp::max(max_depth(&node.left), max_depth(&node.right)),
    }
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode::new(2))),
        right: Some(Box::new(TreeNode::new(3))),
    }));

    println!("Maximum depth of the tree: {}", max_depth(&root));
}
