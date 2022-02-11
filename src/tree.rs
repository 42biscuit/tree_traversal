use rand::prelude::*;

use crate::{
    node::{TreeIndex, TreeNode},
    test::{System, FORCE_APPLIED},
};

pub struct TreeBuilder {
    inputs: usize,
    outputs: usize,
    hidden_layers: usize,
    inner_depth: usize,
}
impl TreeBuilder {
    pub fn new(inputs: usize, outputs: usize, hidden_layers: usize, inner_depth: usize) -> Self {
        TreeBuilder {
            inputs,
            outputs,
            hidden_layers,
            inner_depth,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Tree {
    arena: Vec<Option<TreeNode>>,
    roots: Vec<Option<TreeIndex>>,
}

pub struct trainer<T, A> {
    base: Tree,
    generations: usize,
    population: usize,
    rand_rate: f32,
    test: fn(&mut A, &[f32]) -> Vec<T>, //returns angle,cart a
    test_struct: System,
    test_fail: fn(&[f32]) -> bool,
}
impl<T, A: Clone> trainer<T, A> {
    pub fn new(
        base: TreeBuilder,
        generations: usize,
        population: usize,
        rand_rate: f32,
        test: fn(&mut A, &[f32]) -> Vec<T>,
        test_struct: System,
        test_fail: fn(&[f32]) -> bool,
    ) -> Self {
        trainer {
            base: Tree::new(base),
            generations,
            population,
            rand_rate,
            test,
            test_struct,
            test_fail,
        }
    }
    pub fn train(&mut self) -> Tree {
        let mut end_best = Vec::new();
        let mut pop_pool = vec![(self.base.clone(), 0); self.population];
        let mut learning_rate = 0.0;
        let mut last_best = 1.0;
        'found: for g in 1..self.generations + 1 {
            println!("{}", g);
            learning_rate = 0.8 * (1.5_f32.powf(-0.3 * last_best / 300.0)) + 0.01;
            for i in &mut pop_pool {
                let mut test_st = self.test_struct.clone();
                let mut data = test_st.update_vals(&[0.0]);
                let mut res = Vec::new();

                while ((self.test_fail)(&[test_st.pole_angle(), test_st.cart_pos()])) == false {
                    res = i.0.run_through_bf(&data);
                    if res[0] > res[1] {
                        data = test_st.update_vals(&[FORCE_APPLIED]);
                    } else {
                        data = test_st.update_vals(&[-FORCE_APPLIED]);
                    }
                    if i.1 > 100000 {
                        println!("found a good one {}", i.1);
                        end_best.push(i.0.clone());
                        break 'found;
                    }
                    i.1 += 1;
                }
            }
            for i in 1..self.population {
                let mut j = i;
                while j > 0 && pop_pool[j].1 < pop_pool[j - 1].1 {
                    let buffer = pop_pool[j].clone();
                    pop_pool[j] = pop_pool[j - 1].clone();
                    pop_pool[j - 1] = buffer;
                    j -= 1;
                }
            }

            println!("best:{}", pop_pool[self.population - 1].1);
            last_best = pop_pool[self.population - 1].1 as f32;
            let best = &pop_pool.clone()[self.population - 1];
            end_best.push(best.0.clone());
            if best.1 > 10000 {
                break 'found;
            }
            for i in 0..pop_pool.len() {
                pop_pool[i] = (best.0.evolve(learning_rate), 0);
            }

            /*let chunk = (self.population - self.population % 9) / 9;
            for i in 0..9 {
                for j in 0..chunk {
                    pop_pool[i * chunk + j] = (best.0.evolve(self.rand_rate), 0);
                }
            }*/
        }

        end_best.last().unwrap().clone()
    }
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
                    vec![0.5; next_nodes.len()],
                ))));
            }
            next_nodes = current_nodes.clone();
            current_nodes = vec![];
        }

        for _ in 0..builder.inputs {
            current_nodes.push(Some(a.add_node(TreeNode::new(
                0.0,
                next_nodes.clone(),
                vec![0.5; next_nodes.len()],
            ))));
            a.add_root(*current_nodes.last().unwrap());
        }
        println!("roots: {:?}", a.roots);
        a
    }

    pub fn run_through_bf(&mut self, inputs: &[f32]) -> Vec<f32> {
        let mut iter = self.iter();
        for i in 0..self.roots.len() {
            let mut input = self.node_at_mut(self.roots[i].unwrap()).unwrap();
            input.value = *inputs.get(i).unwrap();
        }
        let mut opts = Vec::new();
        while let Some(res) = iter.next_layer(self) {
            opts = res;
        }
        opts
    }

    pub fn run_through_df(&mut self, inputs: &[f32]) {
        for i in &self.arena {
            println!("next nodes:{:?}", i.as_ref().unwrap().next_nodes);
        }
        for i in 0..self.roots.len() {
            let mut pos_iter = self.iter();
            let mut input = self.node_at_mut(self.roots[i].unwrap()).unwrap();
            input.value = *inputs.get(i).unwrap();
            println!("inmpu val = {}", input.value);
            while let Some(node) = pos_iter.next(&self) {
                let crnt_node = self.node_at(node).unwrap().clone();

                for (next_index, weight) in
                    crnt_node.next_nodes.iter().zip(crnt_node.weights.iter())
                {
                    let mut a = self.node_at_mut(next_index.unwrap()).unwrap();
                    a.value += crnt_node.value * weight;
                    println!(
                        "node: {}   a: {} = {} * {}",
                        node, &a.value, crnt_node.value, weight
                    );
                }
            }
        }
        println!("opt = {}", &self.arena[0].as_ref().unwrap().value);
        /*let a = self.arena.last().unwrap().as_ref().unwrap().value;
        a*/
    }
    fn evolve(&self, rand_rate: f32) -> Self {
        let mut tree = self.clone();
        let mut rng = rand::thread_rng();
        let mut itera = tree.iter();
        while let Some(index) = itera.next(&tree) {
            let node = tree.node_at_mut(index).unwrap();
            for i in 0..node.weights.len() {
                node.weights[i] += rng.gen_range(-1.0..1.0) * rand_rate;
            }
        }
        tree
    }
    pub fn iter(&self) -> PreorderIter {
        PreorderIter::new(self.roots.clone())
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
    next_nodes: Vec<Option<TreeIndex>>,
    crnt: Vec<Option<TreeIndex>>,
}

impl PreorderIter {
    pub fn new(roots: Vec<Option<TreeIndex>>) -> Self {
        PreorderIter {
            crnt: roots,
            next_nodes: vec![],
        }
    }
    pub fn next_layer(&mut self, tree: &mut Tree) -> Option<Vec<f32>> {
        let mut activations = Vec::new();
        self.next_nodes = tree
            .node_at(self.crnt[0].unwrap())
            .unwrap()
            .next_nodes
            .clone();

        while let Some(crnt_index) = self.crnt.pop() {
            let crnt_node = tree.node_at(crnt_index.unwrap()).unwrap().clone();

            for (next_index, weight) in crnt_node.next_nodes.iter().zip(crnt_node.weights.iter()) {
                let mut a = tree.node_at_mut(next_index.unwrap()).unwrap();
                a.value += crnt_node.value * weight;
            }
        }
        for index in &self.next_nodes {
            let a = tree.node_at_mut(index.unwrap()).unwrap();
            a.squish();
            activations.push(a.value)
        }

        self.crnt = self.next_nodes.clone();

        if self.crnt.len() == 0 {
            None
        } else {
            Some(activations)
        }
    }

    pub fn next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        while let Some(node_index) = self.crnt.pop() {
            if let Some(node) = tree.node_at(node_index.unwrap()) {
                for i in &*node.next_nodes {
                    self.crnt.push(*i);
                }

                return node_index;
            }
        }

        None
    } // immutable borrow &Tree ends here*/
}

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
        let mut pos_iter = tree.iter();
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

        let mut pos_iter = tree.iter();
        while let Some(node) = pos_iter.next(&tree) {
            let node = tree.node_at_mut(node).unwrap();
            node.value *= 10.0;
        }

        let mut pos_iter = tree.iter();
        let mut values = Vec::new();
        while let Some(node) = pos_iter.next(&tree) {
            let node = tree.node_at(node).unwrap();
            values.push(node.value);
        }
        assert_eq!(values, vec![10.0, 30.0, 20.0, 60.0, 50.0, 40.0]);
    }
}
