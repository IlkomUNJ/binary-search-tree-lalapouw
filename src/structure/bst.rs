use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

// This package implements a wrapper for BST (Binary Search Tree)
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    // Create a new node with a specific key
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    // Public function to create a new node link with a value
    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let current_node = BstNode::new(value);
        Rc::new(RefCell::new(current_node))
    }

    /**
     * Get a copy of the node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    // Convert a node to a weak reference
    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    // Create a new node with a parent link
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut current_node = BstNode::new(value);
        current_node.parent = Some(BstNode::downgrade(parent));
        Rc::new(RefCell::new(current_node))
    }

    // Add a left child and set its parent to the current node
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    // Add a right child and set its parent to the current node
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    // Search for a node with a matching value in the tree
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        None
    }

    /** Recursively find the minimum value (always to the left in BST) */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    // Find the maximum value (always to the right in BST)
    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root node of the tree, or return self if it has no parent
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * Find the successor of a node according to the BST rules.
     * Returns None if the node is the highest key in the tree.
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        }

        let mut x_node = x_node;
        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let mut temp: BstNodeLink;

        while let Some(ref exist) = y_node {
            if let Some(ref left_child) = exist.borrow().left {
                if BstNode::is_node_match(left_child, x_node) {
                    return Some(exist.clone());
                }
            }

            temp = y_node.unwrap();
            x_node = &temp;
            y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
        }

        None
    }

    // Insert a new node with a key into the tree
    pub fn insert(root: &mut Option<BstNodeLink>, key: i32) {
        match root {
            None => {
                *root = Some(BstNode::new_bst_nodelink(key));
            }
            Some(node) => {
                let mut node_borrow = node.borrow_mut();
                if key < node_borrow.key.unwrap() {
                    if node_borrow.left.is_none() {
                        let new_node = BstNode::new_bst_nodelink(key);
                        new_node.borrow_mut().parent = Some(Rc::downgrade(node));
                        node_borrow.left = Some(new_node);
                    } else {
                        drop(node_borrow); // Release borrow before recursion
                        BstNode::insert(&mut node.borrow_mut().left, key);
                    }
                } else {
                    if node_borrow.right.is_none() {
                        let new_node = BstNode::new_bst_nodelink(key);
                        new_node.borrow_mut().parent = Some(Rc::downgrade(node));
                        node_borrow.right = Some(new_node);
                    } else {
                        drop(node_borrow);
                        BstNode::insert(&mut node.borrow_mut().right, key);
                    }
                }
            }
        }
    }

    // Replace a node in the tree (used for deletion)
    pub fn transplant(root: &mut Option<BstNodeLink>, u: &BstNodeLink, v: Option<BstNodeLink>) {
        if let Some(parent_weak) = &u.borrow().parent {
            if let Some(parent) = parent_weak.upgrade() {
                if Rc::ptr_eq(u, parent.borrow().left.as_ref().unwrap_or(u)) {
                    parent.borrow_mut().left = v.clone();
                } else {
                    parent.borrow_mut().right = v.clone();
                }
            }
        } else {
            *root = v.clone();
        }

        if let Some(ref v_node) = v {
            v_node.borrow_mut().parent = u.borrow().parent.clone();
        }
    }

    // Delete a node from the tree
    pub fn delete(root: &mut Option<BstNodeLink>, z: &BstNodeLink) {
        let z_left = z.borrow().left.clone();
        let z_right = z.borrow().right.clone();

        if z_left.is_none() {
            BstNode::transplant(root, z, z_right);
        } else if z_right.is_none() {
            BstNode::transplant(root, z, z_left);
        } else {
            let mut y = z_right.clone();
            while let Some(ref y_node) = y {
                if y_node.borrow().left.is_some() {
                    let left_child = y_node.borrow().left.clone();
                    y = left_child;
                } else {
                    break;
                }
            }

            if let Some(y_node) = y.clone() {
                if !Rc::ptr_eq(&y_node, &z_right.as_ref().unwrap()) {
                    let y_right = y_node.borrow().right.clone();
                    BstNode::transplant(root, &y_node, y_right);
                    y_node.borrow_mut().right = z_right.clone();
                    if let Some(ref right_child) = y_node.borrow().right {
                        right_child.borrow_mut().parent = Some(Rc::downgrade(&y_node));
                    }
                }

                BstNode::transplant(root, z, y.clone());
                y_node.borrow_mut().left = z_left.clone();
            }
        }
    }

    /**
     * A simpler version of tree_successor that checks if the node is nil
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node) != true {
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;

        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        return Some(y_node.clone().unwrap());
    }

    // Check if a node is "nil" (no parent, no children, and no key)
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none() || x.borrow().left.is_none() || x.borrow().right.is_none() {
                    return true;
                }
                return false;
            }
        }
    }

    // Helper function to check if two nodes are equal by comparing their keys
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    // Check if two nodes are equal
    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * Upgrade a weak reference to a strong reference
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
}
