use crate::{framework::SingleObjective, problems};
use std::sync::Mutex;

pub use coco_rs::{Problem, Suite};

pub mod suits;

#[derive(serde::Serialize)]
pub struct CocoInstance {
    #[serde(skip)]
    problem: Mutex<Problem>,
    function: usize,
    instance: usize,
    dimension: usize,
}

impl CocoInstance {
    pub fn format_name(&self) -> String {
        self.problem.lock().unwrap().id().to_string()
    }
}

impl From<Problem> for CocoInstance {
    fn from(problem: Problem) -> Self {
        CocoInstance {
            function: problem.function_index(),
            instance: problem.instance_index(),
            dimension: problem.dimension_index(),
            problem: Mutex::new(problem),
        }
    }
}

impl problems::Problem for CocoInstance {
    type Encoding = Vec<f64>;
    type Objective = SingleObjective;

    fn evaluate_solution(&self, solution: &Self::Encoding) -> Self::Objective {
        let output = &mut [0.0];
        self.problem
            .lock()
            .unwrap()
            .evaluate_function(solution, output);
        output[0].try_into().unwrap()
    }

    fn name(&self) -> &str {
        "Coco"
    }
}

impl problems::VectorProblem for CocoInstance {
    type T = f64;

    fn dimension(&self) -> usize {
        self.problem.lock().unwrap().dimension()
    }
}

impl problems::LimitedVectorProblem for CocoInstance {
    fn range(&self, dimension: usize) -> std::ops::Range<Self::T> {
        let problem = self.problem.lock().unwrap();
        let range = problem.get_ranges_of_interest()[dimension].clone();

        let (start, end) = range.into_inner();
        start..end
    }
}

impl problems::HasKnownTarget for CocoInstance {
    fn target_hit(&self, _target: SingleObjective) -> bool {
        self.problem.lock().unwrap().final_target_hit()
    }
}

//impl problems::HasKnownOptimum for CocoInstance {
//    fn known_optimum(&self) -> SingleObjective {
//        self.problem.lock().unwrap().final_target_fvalue1()
//    }
//}
