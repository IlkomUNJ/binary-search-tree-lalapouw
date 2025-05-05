mod structure;
mod tool;

use crate::structure::bst::BstNode;
use crate::structure::tree::Node;
use crate::structure::tree::NodeLink;
use crate::structure::bst::BstNodeLink;
use crate::tool::generate_dotfile;
use crate::tool::generate_dotfile_bst;

fn main() {
    // Uncomment to test the old tree structure code
    // test_binary_tree();
    test_binary_search_tree();
}

fn test_binary_search_tree() {
    // Create a root node with key 15
    let rootlink: BstNodeLink = BstNode::new_bst_nodelink(15);
    rootlink.borrow_mut().add_left_child(&rootlink, 6);
    rootlink.borrow_mut().add_right_child(&rootlink, 18);

    // Add nodes to the right subtree
    if let Some(right_tree) = &rootlink.borrow().right {
        right_tree.borrow_mut().add_left_child(right_tree, 17);
        right_tree.borrow_mut().add_right_child(right_tree, 20);
    }

    // Add nodes to the left subtree
    if let Some(left_tree) = &rootlink.borrow().left {
        left_tree.borrow_mut().add_left_child(left_tree, 3);
        left_tree.borrow_mut().add_right_child(left_tree, 7);

        // Add children to node 3
        if let Some(left_subtree) = &left_tree.borrow().left {
            left_subtree.borrow_mut().add_left_child(left_subtree, 2);
            left_subtree.borrow_mut().add_right_child(left_subtree, 4);
        }

        // Add children to node 7
        if let Some(right_subtree) = &left_tree.borrow().right {
            right_subtree.borrow_mut().add_right_child(right_subtree, 13);
            if let Some(grandchild) = &right_subtree.borrow().right {
                grandchild.borrow_mut().add_left_child(grandchild, 9);
            }
        }
    }

    // Insert new values into the tree
    let mut rootlink: Option<BstNodeLink> = None;
    BstNode::insert(&mut rootlink, 42);
    BstNode::insert(&mut rootlink, 17);
    BstNode::insert(&mut rootlink, 68);
    BstNode::insert(&mut rootlink, 9);
    BstNode::insert(&mut rootlink, 33);

    println!("Tree structure has been modified after insertions.");

    // Delete the root node
    if let Some(root_node) = rootlink.clone() {
        BstNode::delete(&mut rootlink, &root_node);
    }

    println!("Tree structure has been modified after deletion.");

    // Output the tree to a DOT file for visualization
    let main_tree_path = "bst_graph.dot";
    generate_dotfile_bst(rootlink.as_ref().unwrap(), main_tree_path);

    // Test tree search for specific keys
    let search_keys = vec![15, 9, 22];
    for &key in search_keys.iter() {
        print!("Tree search result for key {}: ", key);
        if let Some(ref node) = rootlink {
            if let Some(node_result) = node.borrow().tree_search(&key) {
                println!("Found -> {:?}", node_result.borrow().key);
            } else {
                println!("Not found");
            }
        } else {
            println!("Tree is empty");
            break;
        }
    }

    // Get the minimum and maximum values in the tree
    if let Some(ref node) = rootlink {
        let min_node = node.borrow().minimum();
        println!("Minimum value: {:?}", min_node.borrow().key);

        let max_node = node.borrow().maximum();
        println!("Maximum value: {:?}", max_node.borrow().key);
    } else {
        println!("Tree is empty, cannot get min or max");
    }

    // Test getting the root node
    if let Some(ref node) = rootlink {
        let max_node = node.borrow().maximum();
        let root_node = BstNode::get_root(&max_node);
        println!("Root node: {:?}", root_node.borrow().key);
    }

    // Test successor for specific nodes
    let query_keys = vec![
        2,  // min_node, should return its parent Some(3)
        20, // max_node, should return None
        15, // root_node, should return the minimum of its right subtree
        13, // node with empty right child, should return the parent
        9, 7, // other keys
        22  // non-existent key
    ];

    for &key in query_keys.iter() {
        if let Some(ref node_rc) = rootlink {
            if let Some(node) = node_rc.borrow().tree_search(&key) {
                print!("Successor of node ({}) is ", key);
                match BstNode::tree_successor_simpler(&node) {
                    Some(successor) => println!("{:?}", successor.borrow().key),
                    None => println!("No successor found"),
                }
            } else {
                println!("Node with key {} does not exist, failed to get successor", key);
            }
        } else {
            println!("Tree is empty.");
        }        
    }
}

#[allow(dead_code)]
fn test_binary_tree() {
    // Create the root node of the binary tree
    let rootlink: NodeLink = Node::new_nodelink(5);

    // Add a left child with value 3
    rootlink.borrow_mut().add_left_child(&rootlink, 3);
    // Add a right child with value 7
    rootlink.borrow_mut().add_right_child(&rootlink, 7);

    // Output the tree to a DOT file for visualization
    let mut main_tree_path = "prime.dot";
    generate_dotfile(&rootlink, main_tree_path);

    // Add children to the left subtree
    if let Some(left_tree) = &rootlink.borrow().left {
        left_tree.borrow_mut().add_left_child(left_tree, 2);
        left_tree.borrow_mut().add_right_child(left_tree, 4);
    }

    // Add children to the right subtree
    if let Some(right_tree) = &rootlink.borrow().right {
        right_tree.borrow_mut().add_right_child(right_tree, 10);
    }

    // Output the tree again after adding more values
    main_tree_path = "prime_t2.dot";
    generate_dotfile(&rootlink, main_tree_path);

    // Print the depth of the tree
    let recorded_depth = rootlink.borrow().tree_depth();
    println!("Current tree depth: {0}", recorded_depth);

    // Print the number of nodes in the tree
    let total_nodes = rootlink.borrow().count_nodes();
    println!("Number of nodes in the tree: {0}", total_nodes);

    // Count nodes in the right subtree
    let subtree_count = Node::count_nodes_by_nodelink(&rootlink.borrow().right.clone().unwrap(), 0);
    println!("Number of nodes in the right subtree: {0}", subtree_count);

    // Get the sibling of the left subtree
    let _left_subtree_sibling = Node::get_sibling(&rootlink.borrow().left.clone().unwrap());

    // Get a node by its value
    let left_subtree = rootlink.borrow().get_node_by_value(3);
    println!("Left subtree by value: {:?}", left_subtree);

    // Get a node by full properties
    let another_left_subtree = rootlink
        .borrow()
        .get_node_by_full_property(&left_subtree.as_ref().unwrap());
    println!(
        "Left subtree by full property: {:?}",
        another_left_subtree
    );

    // Delete the node with value 3
    let rootlink2 = rootlink.borrow().get_nodelink_copy();
    let flag = rootlink2.borrow_mut().discard_node_by_value(3);
    println!("Status of node deletion: {0}", flag);

    // Output the tree again after deletion
    main_tree_path = "prime_t3.dot";
    generate_dotfile(&rootlink2, main_tree_path);

    // Print the tree depth and node count after deletion
    let depth_now = rootlink2.borrow().tree_depth();
    println!("Depth after deletion: {0}", depth_now);

    let count_now = rootlink2.borrow().count_nodes();
    println!("Node count after deletion: {0}", count_now);

    // Output the tree again for comparison
    main_tree_path = "prime_t4.dot";
    generate_dotfile(&rootlink, main_tree_path);
}
