/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering; // 用于比较大小
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            },
            Some(root) => {
                // 如果root中已经有值，那就直接在root中insert，
                // 节点的insert已经实现了自动查找空的left和right
                root.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match &self.root {
            None => {
                false
            },
            Some(root) => {
                //同上，节点处已经实现了search
                root.search(value)
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        match value.cmp(&self.value) {
            // 其实就是比较大小，只是Ordering可以让不需要操作的情况正常存在
            // 比根节点小的放在左侧，大的放右侧，接下去的每一层都这样实现
            Ordering::Less => {
                match &mut self.left {
                    // 左侧没有节点时创建储存value的新节点，否则在下一级递归调用insert，直到没有节点为止
                    Some(left) => left.insert(value),
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                    },
                }
            },
            Ordering::Greater => {
                match &mut self.right {
                    Some(right) => right.insert(value),
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                    },
                }
            },
            Ordering::Equal => {
                // 二叉树中不需要重复的元素
            }
        }
    }

    fn search(&self, value: T) -> bool {
        //TODO
        match value.cmp(&self.value) {
            Ordering::Less => match &self.left {
                // 如果要找的值比当前节点小，说明符合条件的值在子树的左侧
                Some(ref left) => left.search(value),
                None => false,
            }
            Ordering::Greater => match &self.right {
                Some(ref right) => right.search(value),
                None => false,
            }
            Ordering::Equal => true
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


