use crate::framework::State;

pub trait LogTrigger {
    fn should_log(&mut self, state: &State) -> bool;
}
