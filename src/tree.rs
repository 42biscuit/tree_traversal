use crate::node::{TreeIndex, TreeNode};

pub struct TreeBuilder {
    inputs: usize,
    outputs: usize,
    hidden_layers: usize,
    inner_depth: usize,
}
impl TreeBuilder {
    pub fn new(inputs: usize, outputs: usize, hidden_layers: usize, innder_depth: usize) -> Self {
        TreeBuilder {
            inputs: inputs,
            outputs: outputs,
            hidden_layers: hidden_layers,
            inner_depth: innder_depth,
        }
    }
}

pub struct Tree {
    pub arena: Vec<Option<TreeNode>>,
    roots: Vec<Option<TreeIndex>>,
}

impl Tree {
    ///consumes a TreeBuilder and returns a general net
    pub fn new(builder: TreeBuilder) -> Self {
        let mut a = Tree {
            arena: Vec::new(),
            roots: Vec::new(),
        };
        let mut next_nodes = Vec::new();
        let mut current_nodes = Vec::new();
        for _ in 0..builder.outputs {
            next_nodes.push(Some(a.add_node(TreeNode::new(0.0, vec![], vec![]))))
        }
        for _ in 0..builder.hidden_layers {
            for _ in 0..builder.inner_depth {
                current_nodes.push(Some(a.add_node(TreeNode::new(
                    0.0,
                    next_nodes.clone(),
                    vec![1.0; next_nodes.len()],
                ))));
            }
            next_nodes = current_nodes.clone();
        }
        
        for _ in 0..builder.inputs {
            current_nodes.push(Some(a.add_node(TreeNode::new(
                0.0,
                next_nodes.clone(),
                vec![1.0; next_nodes.len()],
            ))));
            a.add_root(*current_nodes.last().unwrap());
        }
        println!("roots: {:?}",a.roots);
        a
    }
    pub fn run_through(&mut self, inputs: &[f32]) {
        //lets get breath first
        
        for i in &self.arena{
            println!("next nodes:{:?}",i.as_ref().unwrap().next_nodes);
        }
        for i in 0..self.roots.len() {
            let mut pos_iter = self.iter(i);
            let mut input = self.node_at_mut(self.roots[i].unwrap()).unwrap();
            input.value = *inputs.get(i).unwrap();
            println!("inmpu val = {}",input.value);
            while let Some(node) = pos_iter.next(&self) {
                let crnt_node = self.node_at(node).unwrap().clone();

                for (next_index, weight) in
                    crnt_node.next_nodes.iter().zip(crnt_node.weights.iter())
                {
                    let mut a = self.node_at_mut(next_index.unwrap()).unwrap();
                    a.value += crnt_node.value * weight;
                    println!("node: {}   a: {} = {} * {}",node,&a.value,crnt_node.value,weight);
                }
            }
        }
        println!("opt = {}",&self.arena[0].as_ref().unwrap().value);
        /*let a = self.arena.last().unwrap().as_ref().unwrap().value;
        a*/
    }
    pub fn iter(&self, root_index: usize) -> PreorderIter {
        PreorderIter::new(self.roots[root_index])
    }
    ///adds a root or "input" to the tree, from here iterators can be built to traverse the tree
    pub fn add_root(&mut self, root: Option<TreeIndex>) {
        self.roots.push(root);
    }

    pub fn add_node(&mut self, node: TreeNode) -> TreeIndex {
        let index = self.arena.len();
        self.arena.push(Some(node));
        return index;
    }

    /*    pub fn _remove_node_at(&mut self, index: TreeIndex) -> Option<TreeNode> {
        if let Some(node) = self.arena.get_mut(index) {
            node.take()
        } else {
            None
        }
    }*/

    pub fn node_at(&self, index: TreeIndex) -> Option<&TreeNode> {
        return if let Some(node) = self.arena.get(index) {
            node.as_ref()
        } else {
            None
        };
    }

    pub fn node_at_mut(&mut self, index: TreeIndex) -> Option<&mut TreeNode> {
        return if let Some(node) = self.arena.get_mut(index) {
            node.as_mut()
        } else {
            None
        };
    }
}

pub struct PreorderIter {
    stack: Vec<TreeIndex>,
}

impl PreorderIter {
    pub fn new(root: Option<TreeIndex>) -> Self {
        if let Some(index) = root {
            PreorderIter { stack: vec![index] }
        } else {
            PreorderIter { stack: vec![] }
        }
    }
    ///consumes a PreorderIter and iterates over the tree in the Preoirder
    pub fn next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        while let Some(node_index) = self.stack.pop()  {
            if let Some(node) = tree.node_at(node_index) {
                if node.next_nodes.len() == 0 {
                    eprintln!("output value at index {} = {}", node_index, node.value);
                }
                for i in &*node.next_nodes {
                    self.stack.push((*i).unwrap());
                }

                return Some(node_index);
            }
        }

        None
    } // immutable borrow &Tree ends here
}
//used for
/*impl Iterator for PreorderIter {
    type Item = &'a mut TreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            for i in &mut node.next_nodes {
                if let Some(node) = i {
                    self.stack.push(node);
                }
            }
            return Some(node);
        }
        return None;
    }
}*/

#[cfg(test)]
mod tree_test {
    use crate::{
        node::TreeNode,
        tree::{Tree, TreeBuilder},
    };
    #[test]
    fn traversal_test() {
        let mut tree = Tree::new(TreeBuilder {
            inputs: 0,
            outputs: 0,
            hidden_layers: 0,
            inner_depth: 0,
        });
        let a = tree.add_node(TreeNode::new(4.0, vec![], vec![]));
        let b = tree.add_node(TreeNode::new(5.0, vec![], vec![]));
        let f = tree.add_node(TreeNode::new(6.0, vec![], vec![]));
        let d = tree.add_node(TreeNode::new(3.0, vec![], vec![]));
        let c = tree.add_node(TreeNode::new(2.0, vec![Some(a), Some(b), Some(f)], vec![]));
        let e = tree.add_node(TreeNode::new(1.0, vec![Some(c), Some(d)], vec![]));
        tree.add_root(Some(e));
        let mut pos_iter = tree.iter(0);
        let mut values = Vec::new();
        while let Some(node) = pos_iter.next(&tree) {
            let node = tree.node_at(node).unwrap();
            values.push(node.value);
        }
        assert_eq!(values, vec![1.0, 3.0, 2.0, 6.0, 5.0, 4.0]);
    }
    #[test]
    fn tree_mut() {
        let mut tree = Tree::new(TreeBuilder {
            inputs: 0,
            outputs: 0,
            hidden_layers: 0,
            inner_depth: 0,
        });
        let a = tree.add_node(TreeNode::new(4.0, vec![], vec![]));
        let b = tree.add_node(TreeNode::new(5.0, vec![], vec![]));
        let f = tree.add_node(TreeNode::new(6.0, vec![], vec![]));
        let d = tree.add_node(TreeNode::new(3.0, vec![], vec![]));
        let c = tree.add_node(TreeNode::new(2.0, vec![Some(a), Some(b), Some(f)], vec![]));
        let e = tree.add_node(TreeNode::new(1.0, vec![Some(c), Some(d)], vec![]));
        tree.add_root(Some(e));

        let mut pos_iter = tree.iter(0);
        while let Some(node) = pos_iter.next(&tree) {
            let node = tree.node_at_mut(node).unwrap();
            node.value *= 10.0;
        }

        let mut pos_iter = tree.iter(0);
        let mut values = Vec::new();
        while let Some(node) = pos_iter.next(&tree) {
            let node = tree.node_at(node).unwrap();
            values.push(node.value);
        }
        assert_eq!(values, vec![10.0, 30.0, 20.0, 60.0, 50.0, 40.0]);
    }
}
