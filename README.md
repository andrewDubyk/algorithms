# Algorithms
Experiments with RUST and algorithms.

[![Verify](https://github.com/andrewDubyk/algorithms/actions/workflows/verify.yaml/badge.svg)](https://github.com/andrewDubyk/algorithms/actions/workflows/verify.yaml)


Simple library with tests for next alorithms:

- Add two numbers which are represented as linkedlist data structure;
- Count number of `boomerangs` in vector of points [(x,y)], where `x` and `y` are coordinates which are represented in a coordinate plane. A boomerang is a tuple of points `(i, j, k)` such that the distance between `i` and `j` equals the distance between `i` and `k` (the order of the
tuple matters).


## CI/CD - formatting, linting, testing, build.

Latest release status:

Platform | CI Status
---------|:---------
OSX      | [![Release](https://github.com/andrewDubyk/algorithms/actions/workflows/release.yaml/badge.svg)](https://github.com/andrewDubyk/algorithms/actions/workflows/release.yaml)
Linux    | [![Release](https://github.com/andrewDubyk/algorithms/actions/workflows/release.yaml/badge.svg)](https://github.com/andrewDubyk/algorithms/actions/workflows/release.yaml)
Windows  | [![Release](https://github.com/andrewDubyk/algorithms/actions/workflows/release.yaml/badge.svg)](https://github.com/andrewDubyk/algorithms/actions/workflows/release.yaml)

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
