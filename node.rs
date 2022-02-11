pub type TreeIndex = usize;
#[derive(Clone, Debug)]
pub struct TreeNode {
    pub value: f32,
    pub next_nodes: Vec<Option<TreeIndex>>,
    pub weights: Vec<f32>,
}
impl TreeNode {
    pub fn new(value: f32, next_nodes: Vec<Option<TreeIndex>>, weights: Vec<f32>) -> Self {
        TreeNode {
            value: value,
            next_nodes: next_nodes,
            weights: weights,
        }
    }
    ///sigmoid squishificaiton function to keep the avtivation values and weights between 0 and 1
    pub fn squish(&mut self) {
        self.value = 1.0 / (1.0 + 2.71828_f32.powf(-self.value))
    }
}

impl ActivationFuncs<f32> for f32 {
    fn sigmoid_squish(input: &f32) -> Option<f32> {
        Some(1.0 / (1.0 + 2.71828_f32.powf(*input)))
    }
}

trait ActivationFuncs<T> {
    fn sigmoid_squish(input: &T) -> Option<T>;
}
