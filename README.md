# Algorithms
Experiments with RUST and algorithms

[![Verify](https://github.com/andrewDubyk/algorithms/actions/workflows/verify.yaml/badge.svg)](https://github.com/andrewDubyk/algorithms/actions/workflows/verify.yaml)


Simple library with tests for next alorithms:

- Add two numbers which are represented as linkedlist data structure;
- Count number of `boomerangs` in vector of points [(x,y)], where `x` and `y` are coordinates which are represented in a coordinate plane.


## CI/CD

CI/CD checks includes formatting, linting, testing, build.

Latest release CI/CD status:

Platform | CI Status
---------|:---------
OSX      | 
Linux    | [![Release](https://github.com/andrewDubyk/algorithms/actions/workflows/release.yaml/badge.svg)](https://github.com/andrewDubyk/algorithms/actions/workflows/release.yaml)
Windows  | 

For more details check https://github.com/andrewDubyk/algorithms/actions

## Technical details overview

- Implementation for above algorithms could be found in `src` folder in respective files;
- Algorithms logic is coverd with unit tests under `tests` folder;

## Usage

1. Install `rust` toolchain - https://www.rust-lang.org/tools/install
2. Download repository:
```
git clone https://github.com/andrewDubyk/algorithms.git
```

3. Build project:
```
cargo build --release
```

4. Run tests:
```
cargo test
```

## Documentation

Use `cargo doc --open` to generate project documentation.
