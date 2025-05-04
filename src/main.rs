mod structure;
mod tool;

use crate::structure::bst::{BstNode, BstNodeLink};
use crate::tool::generate_dotfile_bst;

fn main() {
    test_binary_search_tree();
}

fn test_binary_search_tree() {
    let rootlink: BstNodeLink = BstNode::new_bst_nodelink(15);
    rootlink.borrow_mut().add_left_child(&rootlink, 6);
    rootlink.borrow_mut().add_right_child(&rootlink, 18);

    // Add right subtree
    let right_subtree: &Option<BstNodeLink> = &rootlink.borrow().right;
    if let Some(right_tree_extract) = right_subtree {
        right_tree_extract
            .borrow_mut()
            .add_left_child(right_tree_extract, 17);
        right_tree_extract
            .borrow_mut()
            .add_right_child(right_tree_extract, 20);
    }

    // Add left subtree
    let left_subtree: &Option<BstNodeLink> = &rootlink.borrow().left;
    if let Some(left_tree_extract) = left_subtree {
        left_tree_extract
            .borrow_mut()
            .add_left_child(left_tree_extract, 3);
        left_tree_extract
            .borrow_mut()
            .add_right_child(left_tree_extract, 7);

        // Add left subtree terminal
        let left_subtree_terminal = &left_tree_extract.borrow().left;
        if let Some(terminal_left_tree_link) = left_subtree_terminal {
            terminal_left_tree_link.borrow_mut().add_left_child(terminal_left_tree_link, 2);
            terminal_left_tree_link.borrow_mut().add_right_child(terminal_left_tree_link, 4);
        }

        // Add 2nd level right subtree of node 7
        let second_right_subtree = &left_tree_extract.borrow().right;
        if let Some(second_right_subtree_link) = second_right_subtree {
            second_right_subtree_link.borrow_mut().add_right_child(second_right_subtree_link, 13);

            let third_left_subtree = &second_right_subtree_link.borrow().right;
            if let Some(third_left_subtree_link) = third_left_subtree {
                third_left_subtree_link.borrow_mut().add_left_child(third_left_subtree_link, 9);
            }
        }
    }

    // Print the tree at this time
    let main_tree_path = "bst_graph.dot";
    generate_dotfile_bst(&rootlink, main_tree_path);

    // Tree search test
    let search_keys = vec![15, 9, 22];

    for &key in search_keys.iter() {
        print!("Tree search result of key {} is ", key);

        if let Some(node_result) = rootlink.borrow().tree_search(&key) {
            println!("found -> {:?}", node_result.borrow().key);
        } else {
            println!("not found");
        }
    }

    // Min test
    let min_node = rootlink.borrow().minimum();
    println!("Minimum result {:?}", min_node.borrow().key);

    // Max test
    let max_node = rootlink.borrow().maximum();
    println!("Maximum result {:?}", max_node.borrow().key);

    // Root node get test
    let root_node = BstNode::get_root(&max_node);
    println!("Root node {:?}", root_node.borrow().key);

    // Successor test
    let query_keys = vec![
        2,   // min_node, should return its parent Some(3)
        20,  // max_node, should return None
        15,  // root_node, should return the minimum of its right tree
        13,  // should return a parent of the node's ancestor if it's a left child of the parent
        9,   // other keys
        22,  // non-existent key
    ];

    for &key in query_keys.iter() {
        if let Some(node) = rootlink.borrow().tree_search(&key) {
            print!("Successor of node ({}) is ", key);

            if let Some(successor) = BstNode::tree_successor_simpler(&node) {
                println!("{:?}", successor.borrow().key);
            } else {
                println!("Not found");
            }
        } else {
            println!("Node with key of {} does not exist, failed to get successor", key)
        }
    }

    // Tree Insert Test
    let new_node_value = 10;
    println!("\nTesting tree_insert with new value: {}", new_node_value);
    BstNode::tree_insert(&rootlink, new_node_value);
    generate_dotfile_bst(&rootlink, "bst_graph_after_insert.dot");

    // Tree Delete Test
    let delete_key = 6;
    println!("\nTesting tree_delete with key: {}", delete_key);
    BstNode::tree_delete(&rootlink, delete_key);
    generate_dotfile_bst(&rootlink, "bst_graph_after_delete.dot");
}

