Changelog
=========

v0.3.0 - ???
-------

- Changed how *random numbers* are generated for the `shuffle` deck method. Instead of making a new generator for each shuffle, each thread now contains its own generator, and that is passed in to each game, and in turn, to each shuffle instance.
- Added median.
- Added ranks.
- Changed majority of the testing section
  - Following WarGame Go system.
- Changed the output stage.
- Updated Rust (the compiler) from 1.1 to 1.5.
- Updated packages from July to January.
- Added `multiplier` cli argument.

v0.2.0 - July 7th, 2015
-------

Major refactoring due to language changes.

- Converted `fmt::String` to `fmt::Display`
- Converted `#[derive(Clone)]` to `#[derive(Clone, Copy)]` on the traits
- Switched from `format!(...).as_slick` to `&obj.to_string`
- Changed the values from `u32` to `i32` since `x - y` could be negative sometimes
- Switching from `debug!` macro to the new `log` macro set (`info!` usually)
- Added debugging packages
    - use `RUST_LOG=info cargo run` to view debugging output
- Split the benchmarking code from `main.rs` into `benchmark.rs`
- Moved `backpring` into `benchmark.rs`
- Converted `range` calls into new `..` syntax
- Replaced `std::Float` math calls to new `f64` calls
- Added 5-decimal numeric precision to `speed_v`
- Reorganized most `use` statements in the file headers
- Swapped *begun* with *started* because English is difficult
- Moved various helper sections into their own functions
- Added statistical functions
- Added documentation
- Cleaned up spacing and overall code


v0.1.0 - December 2014
-------

Honestly, that was six months ago. Who knows.
