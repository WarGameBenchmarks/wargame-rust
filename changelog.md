Changelog
=========

V 0.2.0
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


V 0.1.0
-------

Honestly, that was six months ago. Who knows.