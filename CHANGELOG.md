## [2.0.1] - 2022-09-04

### Fixed

- Workflow should build and run tests based on all featurs 
- Use a slice instead of an array for packing and unpacking data 
- Change tests to mirror the slice as arguments behaviour 

#### Added

- Feature gate tests to run on either `constant_sizeof` or `non_constant_sizeof` features 

## [2.0.0] - 2022-08-04

### Fixed

- `const_fn` function rename 

#### Added

- Feature gate `const_fn` features for rust toolchains that don't support them yet



## [1.0.1] - 2022-07-04

### Fixed

- Calculation of size of data in memory should be done by `ZeroedStore::size_of` 
- Calculation of memory layout size should return a usize from a const fn



## [1.0.0] - 2022-07-04

### Added
- Limit licenses acceptable for the project using `deny.toml` file
- License project under `MPL-2.0`