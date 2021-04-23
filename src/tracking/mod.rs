mod simple;
pub use simple::write_log;

pub mod trigger;
use trigger::*;

struct EvaluationEntry {
    evaluation: u32,
    current_fx: f64,
    best_fx: f64,
}

struct IterationEntry {
    iteration: u32,
    best_fx: f64,
    diversity: f64,
}

pub struct Log {
    eval_trigger: EvalTrigger,
    evaluations: Vec<EvaluationEntry>,
    pending_evaluation: Option<EvaluationEntry>,

    iter_trigger: IterTrigger,
    iterations: Vec<IterationEntry>,
    pending_iteration: Option<IterationEntry>,
}

impl Default for Log {
    fn default() -> Self {
        Log {
            eval_trigger: EvalTrigger::default(),
            evaluations: Vec::new(),
            pending_evaluation: None,

            iter_trigger: IterTrigger::default(),
            iterations: Vec::new(),
            pending_iteration: None,
        }
    }
}

impl Log {
    pub fn new(eval_trigger: EvalTrigger, iter_trigger: IterTrigger) -> Self {
        Log {
            eval_trigger,
            evaluations: Vec::new(),
            pending_evaluation: None,

            iter_trigger,
            iterations: Vec::new(),
            pending_iteration: None,
        }
    }

    pub fn none() -> Self {
        Log {
            eval_trigger: EvalTrigger::none(),
            evaluations: Vec::new(),
            pending_evaluation: None,

            iter_trigger: IterTrigger::none(),
            iterations: Vec::new(),
            pending_iteration: None,
        }
    }

    pub fn log_evaluation(&mut self, evaluation: u32, current_fx: f64, best_fx: f64) {
        let entry = EvaluationEntry {
            evaluation,
            current_fx,
            best_fx,
        };
        let prev = self.evaluations.last();
        if self.eval_trigger.accepts(prev, &entry) {
            self.evaluations.push(entry);
            self.pending_iteration = None;
        } else {
            self.pending_evaluation = Some(entry);
        }
    }

    pub fn log_iteration(&mut self, iteration: u32, best_fx: f64, diversity: f64) {
        let entry = IterationEntry {
            iteration,
            best_fx,
            diversity,
        };
        let prev = self.iterations.last();
        if self.iter_trigger.accepts(prev, &entry) {
            self.iterations.push(entry);
            self.pending_iteration = None;
        } else {
            self.pending_iteration = Some(entry);
        }
    }

    fn finalize(&mut self) {
        if let Some(evaluation) = self.pending_evaluation.take() {
            self.evaluations.push(evaluation);
        }
        if let Some(iteration) = self.pending_iteration.take() {
            self.iterations.push(iteration);
        }
    }

    fn clear(&mut self) {
        self.evaluations.clear();
        self.pending_evaluation = None;
        self.iterations.clear();
        self.pending_iteration = None;
    }
}
