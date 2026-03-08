# My Rust playground

This is my personal little playground for Rust projects and coding practice.

All content is freely available, but probably not production-ready...

## Installation

### Regular

Just clone the repo and build/run like usual.

### Nix

Nix (and NixOS) installation is done through the [devenv](https://devenv.sh).

Step 1: Install `devenv` if not already installed:

```nix
environment.systemPackages = [
    pkgs.devenv
];
```

Installation instructions are available at the devenv site.

Step 2: Clone the repo

Step 3: In the project folder, activate the development shell:

```bash
devenv shell
```

This should install the necessary toolchains to build and run the project.

## Finished projects

The following projects are finished, in the sense that the basic functionality is there and that I probably will not continue them. "Finished" does not entail production-ready.

-   `rng-provider`: A trait for pseudo-random number generator providers in Rust. Affords both synchronous and asynchronous implementations through a feature gate, and comes with a default implementation for Unix-like systems using the `getrandom` syscall in an external ABI block.
