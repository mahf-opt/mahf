use std::marker::PhantomData;

use better_any::{Tid, TidAble};
use mahf::{
    identifier::Global, problems::Evaluate, Configuration, CustomState, Individual, Problem,
    SingleObjective, State,
};

fn main() {
    struct SomeProblem;
    impl Problem for SomeProblem {
        type Encoding = ();
        type Objective = SingleObjective;
        fn name(&self) -> &str {
            unimplemented!()
        }
    }
    struct ParameterWithLifetime<'a>(PhantomData<&'a ()>);

    #[derive(Tid)]
    struct EvaluatorToSomeProblem<'a> {
        #[allow(dead_code)]
        parameter_without_default: ParameterWithLifetime<'a>,
    }

    impl<'a> EvaluatorToSomeProblem<'a> {
        pub fn new() -> Self {
            Self {
                parameter_without_default: ParameterWithLifetime(PhantomData),
            }
        }
    }

    impl<'a> CustomState<'a> for EvaluatorToSomeProblem<'a> {}
    impl<'a> Evaluate for EvaluatorToSomeProblem<'a> {
        type Problem = SomeProblem;
        fn evaluate(
            &mut self,
            _problem: &Self::Problem,
            _state: &mut State<Self::Problem>,
            _individuals: &mut [Individual<Self::Problem>],
        ) {
            unimplemented!()
        }
    }

    let config = Configuration::builder().evaluate::<Global>().build();

    config
        .optimize_with(&SomeProblem, |state| {
            state.insert_evaluator::<Global>(EvaluatorToSomeProblem::new());
            Ok(())
        })
        .unwrap();
}
