# The Framework

## Configurations

Every heuristic in MAHF is composed of nested [Component](components::Component)s.
A complete set is then called [Configuration].
There exist both structural ([Block](components::Block), [Loop](components::Loop)) and operator components ([Initializer](crate::operators::initialization::Initializer), [Generator](crate::operators::generation::Generator)).

Structural components are used to describe concepts such as sequential evaluation or loops.
They can be found in the [components] module, and there is a [ConfigurationBuilder] which simplifies their usage.

Operator components add actual functionality, such as initialization of the population and generation of a new population. These can be found in the [crate::operators] module.

The [conditions] are similar to components, but for branching or loop conditions.

### Example

```rust
# fn example() -> mahf::framework::Configuration<mahf::problems::coco_bound::CocoInstance> {
use mahf::{
    framework::Configuration,
    operators::*,
};

let params = todo!("your parameter");
#
# let params = mahf::heuristics::iwo::Parameters {
#     initial_population_size: 5,
#     max_population_size: 20,
#     min_number_of_seeds: 0,
#     max_number_of_seeds: 5,
#     initial_deviation: 0.5,
#     final_deviation: 0.001,
#     modulation_index: 3,
# };

Configuration::builder()
    .do_(initialization::RandomSpread::new_init(
        params.initial_population_size,
    ))
    .do_(evaluation::SerialEvaluator::new())
    .while_(termination::FixedIterations::new(100), |conf| {
        conf.do_(selection::DeterministicFitnessProportional::new(
                params.min_number_of_seeds,
                params.max_number_of_seeds,
            ))
            .do_(generation::mutation::IWOAdaptiveDeviationDelta::new(
                params.initial_deviation,
                params.final_deviation,
                params.modulation_index,
            ))
            .do_(evaluation::SerialEvaluator::new())
            .do_(replacement::MuPlusLambda::new(params.max_population_size))
    })
    .build()
# }
```

## Components

Every component has to implement the [Component](components::Component) trait, which looks like this:
```ignore
pub trait Component<P: Problem>: AnyComponent {
    fn execute(&self, problem: &P, state: &mut State);

    fn initialize(&self, problem: &P, state: &mut State) { ... }
}
```

The `initialize` function will be called exactly once for each component.
It allows a component to check for or add state, a concept described in the [state] module documentation.

The `execute` function provides the actual implementation of the component and contains its logic.
For structural components this means orchestrating the execution of its children, while operators get to perform their respective job, such as generating a new population.

For examples check out the source code of the [crate::operators] and [crate::framework::components] modules.

## State

State management is crucial in a modular framework like MAHF and is documented in the [state] module.

## Problems

In order to understand MAHF's component model, it is also important to understand how problems are represented. For this, check out the [crate::problems] module.

## Execution

Once you have obtained both a [Configuration] and a [Problem](crate::problems::Problem), you can run your heuristic on that problem using the [run] function. It returns the final [State](state::State) of the heuristic, from which you can obtain the final results, and if you have added a logger (see [crate::tracking]) a log of the entire execution.
