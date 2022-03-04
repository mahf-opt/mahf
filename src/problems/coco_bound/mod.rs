use crate::problems;
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

    fn evaluate(&self, solution: &Self::Encoding) -> f64 {
        let output = &mut [0.0];
        self.problem
            .lock()
            .unwrap()
            .evaluate_function(solution, output);
        output[0]
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

    fn known_optimum(&self) -> f64 {
        unimplemented!()
    }
}
