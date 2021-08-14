# Modular Heuristics

A framework for modular composition of metaheuristics.

![MAHF module system][module_system]

Every run begins with the [components::Initialization] which initializes the population followed by a [components::Postprocess] which should initialized required custom state.

After the initialization is complete, the main optimization loop will be executed. This consists of one selection, one or many generations followed by the replacement. Whether the loop should continue will then be decided by the [components::Termination].

## Main Loop

First of the [components::Selection] will select individuals from the population. It is up to the selection itself to decide which ones how often and how many over all will be selected.

The next and most complex step is the generation. The [components::Generation] will be provided with the selected individuals solutions and can then generate new solutions based up on these. To allow more modularity, we are not limited to just one generation. Instead we can have many and then let a scheduler decide which ones and in which order they should be invoked.

Lastly the replacement decides which of the old and which of the new solutions should be kept.

[comment]: # (These will be overridden in the code for rustdoc!)
[module_system]: MAHF-module-system.svg

## Individual

An [Individual] is simply there to erase a solutions type and keep track of the solutions fitness. Components which operate with [Individual]s usually do not depend on the specific problem instance and do not need to access the solution. Components which are problem dependent (namely [components::Initialization] and [components::Generation]) will operate on the solutions directly.

## State

While all the components in MAHF are basically pure functions, all additional state is stored in the [State] object. This one provides some common state that is always being tracked by the framework, but it also allows keeping track of [CustomState].

[CustomState] should be added in the post-initialization component. After it has been added it can be accessed and mutated by all components. It can store any kind of data and supports logging as well.

## Running

Simply call the [run] function with a [Problem], a [Configuration] and [some additional parameters](run) to start the optimization process.




