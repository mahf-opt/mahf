use crate::framework::{CustomState, State};

#[derive(Default)]
pub struct LoggerCriteria {
    set: Vec<Box<dyn Criteria>>,
}

impl CustomState for LoggerCriteria {}

impl LoggerCriteria {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, criteria: impl Criteria + 'static) -> Self {
        self.add(criteria);
        self
    }

    pub fn add(&mut self, criteria: impl Criteria + 'static) {
        // TODO: check that the criteria is unique
        self.set.push(Box::new(criteria));
    }

    pub fn evaluate(&mut self, state: &State) -> bool {
        let mut log = false;
        for criteria in &mut self.set {
            log |= criteria.evaluate(state);
        }
        log
    }
}

pub trait Criteria {
    fn evaluate(&mut self, state: &State) -> bool;
}

impl<F> Criteria for F
where
    F: FnMut(&State) -> bool,
{
    fn evaluate(&mut self, state: &State) -> bool {
        (self)(state)
    }
}

pub struct OnNthIteration(pub usize);
pub struct OnImprovement {}
