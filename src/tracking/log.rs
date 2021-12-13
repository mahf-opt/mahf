use crate::{framework::State, tracking::trigger::*};
use crate::operators::custom_states::DiversityState;

pub struct CustomLog {
    pub name: &'static str,
    pub value: f64,
}

pub struct EvaluationEntry {
    pub evaluation: u32,
    pub current_fx: f64,
    pub best_fx: f64,
    pub custom: Vec<CustomLog>,
}

pub struct IterationEntry {
    pub iteration: u32,
    pub best_fx: f64,
    pub diversity: f64,
    pub custom: Vec<CustomLog>,
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
    /// Creates the default logger
    ///
    /// See [EvalTrigger::default] and [IterTrigger::default] for details.
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
    /// Create a logger with custom triggers.
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

    /// Create a logger which logs only the final entries.
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

    pub fn final_best_fx(&self) -> f64 {
        self.evaluations.last().unwrap().best_fx
    }

    pub fn final_iteration(&self) -> &IterationEntry {
        self.iterations.last().unwrap()
    }

    pub fn final_evaluation(&self) -> &EvaluationEntry {
        self.evaluations.last().unwrap()
    }

    pub fn iterations(&self) -> &[IterationEntry] {
        &self.iterations
    }

    pub fn evaluations(&self) -> &[EvaluationEntry] {
        &self.evaluations
    }

    /// Log an evaluation
    pub fn log_evaluation(&mut self, state: &State, current_fx: f64) {
        let entry = EvaluationEntry {
            current_fx,
            evaluation: state.evaluations,
            best_fx: state.best_so_far.into(),
            custom: state.custom.collect_evaluation_log(),
        };
        let prev = self.evaluations.last();
        if self.eval_trigger.accepts(prev, &entry) {
            self.evaluations.push(entry);
            self.pending_iteration = None;
        } else {
            self.pending_evaluation = Some(entry);
        }
    }

    /// Log an iteration.
    pub fn log_iteration(&mut self, state: &State) {
        let entry = IterationEntry {
            iteration: state.iterations,
            best_fx: state.best_so_far.into(),
            diversity: state.custom.get::<DiversityState>().diversity, //TODO: this might cause trouble
            custom: state.custom.collect_iteration_log(),
        };
        let prev = self.iterations.last();
        if self.iter_trigger.accepts(prev, &entry) {
            self.iterations.push(entry);
            self.pending_iteration = None;
        } else {
            self.pending_iteration = Some(entry);
        }
    }

    /// Ensures that the last iteration / evaluation gets logged.
    pub(crate) fn finalize(&mut self) {
        if let Some(evaluation) = self.pending_evaluation.take() {
            self.evaluations.push(evaluation);
        }
        if let Some(iteration) = self.pending_iteration.take() {
            self.iterations.push(iteration);
        }
    }

    /// Removes all log entries.
    pub fn clear(&mut self) {
        self.evaluations.clear();
        self.pending_evaluation = None;
        self.iterations.clear();
        self.pending_iteration = None;
    }
}
