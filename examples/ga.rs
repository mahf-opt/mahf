// use mahf::prelude::*;
// use framework::Configuration;

fn main() {
    // Configuration::builder()
    //     .do_(initialization::RandomSpread::new_init(population_size))
    //     .evaluate_sequential()
    //     .update_best_individual()
    //     .do_(ga::ga(
    //         ga::Parameters {
    //             selection: selection::Tournament::new(population_size, tournament_size),
    //             crossover: generation::recombination::UniformCrossover::new_both(pc),
    //             mutation: generation::mutation::FixedDeviationDelta::new(deviation),
    //             archive: None,
    //             replacement: replacement::Generational::new(population_size),
    //         },
    //         termination,
    //         logger,
    //     ))
    //     .build();
    //
    // Configuration::builder()
    //     .while_(termination, |builder| {
    //         builder
    //             .do_(selection)
    //             .do_(crossover)
    //             .do_(mutation)
    //             .evaluate_sequential()
    //             .update_best_individual()
    //             .do_(replacement)
    //     })
    //     .build();
}
