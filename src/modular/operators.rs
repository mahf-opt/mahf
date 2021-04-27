use crate::{
    modular::{components::*, Individual, Solution, State},
    problem::{LimitedVectorProblem, Problem},
};
use rand::{seq::SliceRandom, Rng};
use rand_distr::Distribution;

//                      //
//    Initialization    //
//                      //

pub struct RandomSpreadInitialization {
    /// Size of the initial population
    pub initial_population_size: u32,
}

impl<P> Initialization<P> for RandomSpreadInitialization
where
    P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    fn initialize(&mut self, problem: &P, population: &mut Vec<Solution>) {
        let rng = &mut rand::thread_rng();
        for _ in 0..self.initial_population_size {
            let solution = (0..problem.dimension())
                .map(|d| rng.gen_range(problem.range(d)))
                .collect::<Solution>();
            population.push(solution);
        }
    }
}

//                      //
//      Selection       //
//                      //

pub struct EsSelection {
    /// Offspring per iteration
    pub lambda: u32,
}

impl Selection for EsSelection {
    fn select<'p>(
        &mut self,
        _state: &mut State,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        let rng = &mut rand::thread_rng();
        for _ in 0..self.lambda {
            selection.push(population.choose(rng).unwrap());
        }
    }
}

pub struct IwoSelection {
    /// Minimum number of seeds per plant per iteration
    pub min_number_of_seeds: u32,
    /// Maximum number of seeds per plant per iteration
    pub max_number_of_seeds: u32,
}

impl Selection for IwoSelection {
    fn select<'p>(
        &mut self,
        _state: &mut State,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        #[rustfmt::skip]
        let best: f64 = population.iter().map(Individual::fitness).min().unwrap().into();
        #[rustfmt::skip]
        let worst: f64 = population.iter().map(Individual::fitness).max().unwrap().into();

        for plant in population.iter() {
            let bonus: f64 = (plant.fitness.into() - worst) / (best - worst);
            let bonus_seeds = (self.max_number_of_seeds - self.min_number_of_seeds) as f64;
            let num_seeds = self.min_number_of_seeds
                + if bonus.is_nan() {
                    // best ≈ worst, thus we picked a generic value
                    (0.5 * bonus_seeds).floor() as u32
                } else {
                    (bonus * bonus_seeds).floor() as u32
                };
            assert!(num_seeds <= self.max_number_of_seeds);

            for _ in 0..num_seeds {
                selection.push(plant);
            }
        }
    }
}

//                      //
//      Generation      //
//                      //

pub struct FixedGeneration {
    /// Standard Deviation for the mutation
    pub deviation: f64,
}

impl<P> Generation<P> for FixedGeneration
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate(
        &mut self,
        _state: &mut State,
        _problem: &P,
        parents: &mut Vec<&Solution>,
        offspring: &mut Vec<Solution>,
    ) {
        let rng = &mut rand::thread_rng();
        let distribution = rand_distr::Normal::new(0.0, self.deviation).unwrap();

        for parent in parents {
            let solution = parent
                .iter()
                .map(|x| x + distribution.sample(rng))
                // TODO: How should clamping work?
                //.map(|x| x.clamp(*problem.range.start(), *problem.range.end()))
                .collect::<Solution>();
            offspring.push(solution);
        }
    }
}

pub struct AdaptiveGeneration {
    /// Initial standard deviation for the mutation
    pub initial_deviation: f64,
    /// Final standard deviation for the mutation
    ///
    /// Must not be larger than `initial_deviation`.
    pub final_deviation: f64,
    /// Modulation index for the standard deviation.
    pub modulation_index: u32,
}

impl<P> Generation<P> for AdaptiveGeneration
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate(
        &mut self,
        state: &mut State,
        _problem: &P,
        parents: &mut Vec<&Solution>,
        offspring: &mut Vec<Solution>,
    ) {
        let rng = &mut rand::thread_rng();

        let deviation = self.final_deviation
            + (1.0 - state.progress).powi(self.modulation_index as i32)
                * (self.initial_deviation - self.final_deviation);
        let distribution = rand_distr::Normal::new(0.0, deviation).unwrap();

        for parent in parents {
            let solution = parent
                .iter()
                .map(|x| x + distribution.sample(rng))
                // TODO: Clamping
                //.map(|x| x.clamp(*problem.range.start(), *problem.range.end()))
                .collect::<Solution>();
            offspring.push(solution);
        }
    }
}

//                      //
//      Replacement     //
//                      //

pub struct FittestReplacement {
    /// Limit to population growth
    pub max_population_size: u32,
}

impl Replacement for FittestReplacement {
    fn replace(
        &mut self,
        _state: &mut State,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    ) {
        population.extend(offspring.drain(..));
        population.sort_unstable_by_key(Individual::fitness);
        population.truncate(self.max_population_size as usize);
    }
}

//                      //
//      Termination     //
//                      //

pub struct FixedIterationsTermination {
    /// Maximum number of iterations
    pub max_iterations: u32,
}

impl Termination for FixedIterationsTermination {
    fn terminate(&mut self, state: &mut State) -> bool {
        state.progress = state.iterations as f64 / self.max_iterations as f64;
        state.iterations >= self.max_iterations
    }
}

pub struct FixedEvaluationsTermination {
    /// Maximum number of evaluations
    pub max_evaluations: u32,
}

impl Termination for FixedEvaluationsTermination {
    fn terminate(&mut self, state: &mut State) -> bool {
        state.progress = state.evaluations as f64 / self.max_evaluations as f64;
        state.evaluations >= self.max_evaluations
    }
}
