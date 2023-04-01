use std::ops::RangeInclusive;

use crate::{
    framework::SingleObjective,
    problems::{self, Evaluator},
    state::common::EvaluatorInstance,
};

pub use coco_rs::{Problem, Suite};

pub mod suits;

#[derive(serde::Serialize)]
pub struct CocoInstance {
    function_idx: usize,
    instance_idx: usize,
    dimension_idx: usize,

    name: String,
    dimension: usize,
    ranges_of_interest: Vec<RangeInclusive<f64>>,
    final_target_value: f64,
}

impl CocoInstance {
    pub fn format_name(&self) -> String {
        self.name.clone()
    }

    fn from(problem: &Problem) -> Self {
        let name = problem.id().to_string();
        let dimension = problem.dimension();
        let ranges_of_interest = problem.get_ranges_of_interest();
        let final_target_value = problem.final_target_value();

        CocoInstance {
            function_idx: problem.function_index(),
            instance_idx: problem.instance_index(),
            dimension_idx: problem.dimension_index(),

            name,
            dimension,
            ranges_of_interest,
            final_target_value,
        }
    }
}

impl problems::Problem for CocoInstance {
    type Encoding = Vec<f64>;
    type Objective = SingleObjective;

    fn name(&self) -> &str {
        "Coco"
    }

    fn default_evaluator<'a>(&self) -> EvaluatorInstance<'a, Self> {
        unimplemented!("the evaluator has to be inserted manually")
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
        let range = self.ranges_of_interest[dimension].clone();

        let (start, end) = range.into_inner();
        start..end
    }
}

impl problems::HasKnownTarget for CocoInstance {
    fn target_hit(&self, target: SingleObjective) -> bool {
        target.value() <= self.final_target_value
    }
}

struct CocoEvaluator<'s> {
    pub problem: Problem<'s>,
}

impl Evaluator for CocoEvaluator<'_> {
    type Problem = CocoInstance;

    fn evaluate(
        &mut self,
        _problem: &Self::Problem,
        _state: &mut crate::state::State,
        individuals: &mut [crate::framework::Individual<Self::Problem>],
    ) {
        for individual in individuals {
            let mut out = [0.0];
            self.problem
                .evaluate_function(individual.solution(), &mut out);
            individual.evaluate(SingleObjective::try_from(out[0]).unwrap())
        }
    }
}
