use crate::problems::coco::Instance;

pub mod toy;

pub type SuiteGenerator = fn(function: usize, instance: usize, dimension: usize) -> Instance;

pub struct Suite {
    functions: Vec<usize>,
    next_function: usize,
    instances: Vec<usize>,
    next_instance: usize,
    dimensions: Vec<usize>,
    next_dimension: usize,
    generator: SuiteGenerator,
}

impl Suite {
    pub fn new(
        functions: Vec<usize>,
        instances: Vec<usize>,
        dimensions: Vec<usize>,
        generator: SuiteGenerator,
    ) -> Self {
        Suite {
            functions,
            next_function: 0,
            instances,
            next_instance: 0,
            dimensions,
            next_dimension: 0,
            generator,
        }
    }

    pub fn functions(&self) -> &[usize] {
        &self.functions
    }

    pub fn instances(&self) -> &[usize] {
        &self.instances
    }

    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    fn current_instance(&self) -> Instance {
        (self.generator)(self.next_function, self.next_instance, self.next_dimension)
    }

    pub fn next_instance(&mut self) -> Option<Instance> {
        if self.next_function == self.functions.len()
            && self.next_instance == self.instances.len()
            && self.next_dimension == self.dimensions.len()
        {
            return None;
        }

        if self.next_instance < self.instances.len() {
            self.next_instance += 1;
            return Some(self.current_instance());
        }

        if self.next_function < self.functions.len() {
            self.next_function += 1;
            return Some(self.current_instance());
        }

        if self.next_dimension < self.dimensions.len() {
            self.next_dimension += 1;
            return Some(self.current_instance());
        }

        unreachable!()
    }
}

impl Iterator for Suite {
    type Item = Instance;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_instance()
    }
}
