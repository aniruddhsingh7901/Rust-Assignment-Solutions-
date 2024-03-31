struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => 1 + std::cmp::max(max_depth(node.left.as_ref()), max_depth(node.right.as_ref())),
    }
}
