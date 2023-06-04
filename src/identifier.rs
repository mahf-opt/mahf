use serde::Serialize;
use trait_set::trait_set;

trait_set! {
    pub trait Identifier = Default + Copy + Clone + Serialize + Send + Sync + 'static;
}

macro_rules! identifier {
    ($name:ident) => {
        /// A default identifier to distinguish between components of the same type.
        #[derive(Default, Copy, Clone, Serialize)]
        pub struct $name;
    };
}

identifier!(Global);

identifier!(A);
identifier!(B);
identifier!(C);
identifier!(D);
identifier!(E);

identifier!(I0);
identifier!(I1);
identifier!(I2);
identifier!(I3);
identifier!(I4);
