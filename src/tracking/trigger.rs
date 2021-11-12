use crate::tracking::log::{EvaluationEntry, IterationEntry};

#[derive(Debug, Clone, Copy)]
pub struct EvalTrigger {
    /// Logs every evaluation where `best` improves.
    pub improvement: bool,
    /// Logs every nth evaluation.
    pub interval: Option<u32>,
}

#[derive(Debug, Clone, Copy)]
pub struct IterTrigger {
    /// Logs every iteration where `best` improves.
    pub improvement: bool,
    /// Logs every nth iteration.
    pub interval: Option<u32>,
}

impl Default for EvalTrigger {
    /// A trigger which only logs improvements.
    fn default() -> Self {
        EvalTrigger {
            improvement: true,
            interval: None,
        }
    }
}

impl Default for IterTrigger {
    /// A trigger which only logs improvements.
    fn default() -> Self {
        IterTrigger {
            improvement: true,
            interval: None,
        }
    }
}

impl EvalTrigger {
    /// Does not log anything.
    pub fn none() -> Self {
        EvalTrigger {
            improvement: false,
            interval: None,
        }
    }

    pub(super) fn accepts(&self, prev: Option<&EvaluationEntry>, new: &EvaluationEntry) -> bool {
        if prev.is_none() {
            return true;
        }

        if self.improvement && prev.unwrap().best_fx > new.best_fx {
            return true;
        }

        if let Some(interval) = self.interval {
            if new.evaluation % interval == 0 {
                return true;
            }
        }

        false
    }
}

impl IterTrigger {
    /// Does not log anything.
    pub fn none() -> Self {
        IterTrigger {
            improvement: false,
            interval: None,
        }
    }

    pub(super) fn accepts(&self, prev: Option<&IterationEntry>, new: &IterationEntry) -> bool {
        if prev.is_none() {
            return true;
        }

        if self.improvement && prev.unwrap().best_fx > new.best_fx {
            return true;
        }

        if let Some(interval) = self.interval {
            if new.iteration % interval == 0 {
                return true;
            }
        }

        false
    }
}
