use crate::{framework::SingleObjective, problems};
use std::sync::{Arc, Mutex};

pub use coco_rs::{Problem, Suite};

pub mod suits;

#[derive(serde::Serialize)]
pub struct CocoInstance {
    #[serde(skip)]
    problem: Arc<Mutex<Suite>>,
    function: usize,
    instance: usize,
    dimension: usize,
}

impl CocoInstance {
    pub fn format_name(&self) -> String {
        todo!()
    }

    fn from(suite: &Arc<Mutex<Suite>>, problem: Problem) -> Self {
        CocoInstance {
            function: problem.function_index(),
            instance: problem.instance_index(),
            dimension: problem.dimension_index(),
            problem: Arc::clone(suite),
        }
    }
}

impl problems::Problem for CocoInstance {
    type Encoding = Vec<f64>;
    type Objective = SingleObjective;

    fn evaluate_solution(&self, solution: &Self::Encoding) -> Self::Objective {
        unimplemented!()
    }

    fn name(&self) -> &str {
        "Coco"
    }
}

impl problems::VectorProblem for CocoInstance {
    type T = f64;

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl problems::LimitedVectorProblem for CocoInstance {
    fn range(&self, dimension: usize) -> std::ops::Range<Self::T> {
        todo!()
    }
}

impl problems::HasKnownTarget for CocoInstance {
    fn target_hit(&self, _target: SingleObjective) -> bool {
        todo!()
    }
}
