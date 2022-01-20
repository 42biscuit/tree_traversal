mod node;
mod tree;
use node::TreeNode;
use tree::{Tree, TreeBuilder};
fn main() {
    let mut tree = Tree::new(TreeBuilder::new(2, 1, 1, 2));

     tree.run_through(&[0.2,0.2]);

    /*let mut tree = Tree::new(TreeBuilder::new(0, 0, 0, 0));
    let a = tree.add_node(TreeNode::new(4.0, vec![], vec![]));
    let b = tree.add_node(TreeNode::new(5.0, vec![], vec![]));
    let f = tree.add_node(TreeNode::new(6.0, vec![], vec![]));
    let d = tree.add_node(TreeNode::new(3.0, vec![], vec![]));
    let c = tree.add_node(TreeNode::new(2.0, vec![Some(a), Some(b), Some(f)], vec![]));
    let e = tree.add_node(TreeNode::new(1.0, vec![Some(c), Some(d)], vec![]));
    tree.add_root(Some(e));
    let mut pos_iter = tree.iter(0);
    while let Some(node) = pos_iter.next(&tree) {
        let node = tree.node_at(node).unwrap();

        println!("{}", node.value);
    }
    let mut pos_iter = tree.iter(0);
    while let Some(i) = pos_iter.next(&tree) {
        let node = tree.node_at_mut(i).unwrap();
        node.value *= 10.0;
    }
    let mut pos_iter = tree.iter(0);
    while let Some(node) = pos_iter.next(&tree) {
        let node = tree.node_at(node).unwrap();

        println!("{}", node.value);
    }*/
}
