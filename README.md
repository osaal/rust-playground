# My Rust playground

This is my personal little playground for Rust projects and coding practice.

All content is freely available, but probably not production-ready...

Finished projects:

[x] `rng-provider`: A trait for pseudo-random number generator providers in Rust. Affords both synchronous and asynchronous implementations through a feature gate, and comes with a default implementation for Unix-like systems using the `getrandom` syscall in an external ABI block.