Changelog
=========

- Converted `fmt::String` to `fmt::Display`
- Converted `#[derive(Clone)]` to `#[derive(Clone, Copy)]` on the traits
- Switched from `format!(...).as_slick` to `&obj.to_string` 
- Changed the values from `u32` to `i32` since `x - y` could be negative sometimes
- Switching from `debug!` macro to the new `log` macro set (`info!` usually)
- Added debugging packages
    - use `RUST_LOG=info cargo run` to view debugging output 