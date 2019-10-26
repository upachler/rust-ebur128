# rust-ebur128
![Build Status](https://api.travis-ci.org/upachler/rust-ebur128.svg?branch=master)

Rust bindings for libebur128

rust-ebur128 is a [Rust](https://rust-lang.org) crate that provides loudness metering functionatily conforming to the EBU R 128 standard for loudness normalisation. 
The crate does not implemnet the standard itself, but instead provides bindings to the original [`libebur128`](https://github.com/jiixyj/libebur128).

# Building
Instead of a using a potentially installed `libebur128`, the `build.rs` build script downloads sources from github and compiles them using cmake. Currently, we link the static version of the library.
In order vor all of this to work, you'll need these prerequisites installed:
 * A C compiler (gcc/clang on unix systems, Visual Studio on windows)
 * cmake from cmake.org
 * Rust (of course!), tested with 1.38 (stable)
After that, simply run `cargo build` to build the crate.

# Using the crate
Because there currently is no offical version yet, you'll have to bind the github repository directly as dependency.
 
# License
Like the original `libebur128`, this crate is under [MIT license](https://opensource.org/licenses/MIT).
