# State Management

State management is a central part of MAHF.
Because all components are immutable, they have to store their state in a shared state object.
This object is called [State].

Every type of state is defined as a struct.
The struct itself doubles as key for the state.

Suppose you want to define some new state, which lets you store an adaptive parameter.
You would start by defining the state like this:

```rust
use mahf::framework::state::CustomState;
use mahf::derive_deref::{Deref, DerefMut};
use mahf::serde::Serialize;

#[derive(Default, Debug, Deref, DerefMut, Serialize)]
struct Temperature(pub f64);

impl CustomState for Temperature {}
```

Now you can use it in your component:

```ignore
// TODO: Insert example of Simulated Annealing
# struct Ignore;
```

## Custom State

The [CustomState] trait serves as a marker for custom state.
You can take a look at its documentation to get a list off all state types provided by MAHF.
If you have custom state representing a single value, it is recommended to also derive [Deref](derive_deref::Deref), [DerefMut](derive_deref::DerefMut) and [serde::Serialize].

## Mutable Access

It is often necessary to mutate multiple types of state simultaneously.
MAHF provides two ways of achieving this.

### [State::get_multiple_mut]

The [State::get_multiple_mut] function works just like [State::get_mut], but you can pass a tuple of states.
This function is best suited whey you need access to a few states and request those at the beginning of your function.
If you request access to the same type twice, you'll immediately get an error.
This this function is the simpler and saver choice for most use cases.

### [State::get_states_mut]

This function provides access to [MutState].
[State::get_multiple_mut] should be preferred unless you need the additional flexibility this provides.
[MutState] provides the same functionality as [State], operates similarly [RefCell](std::cell::RefCell).
This way you can borrow multiple types mutably, but it will panic the moment you borrow a type you've borrowed before.
It is a bit stricter than [RefCell](std::cell::RefCell) however, so be sure to check out it's documentation.

### Common State

The module for [common] state contains state which is required by most heuristics.
Together with [operators::custom_state](crate::operators::custom_state) it serves as a good example on how to write your own state types.
