# An `unlikely` branch hint for stable Rust

[![crates.io](https://img.shields.io/crates/v/unlikely.svg)](https://crates.io/crates/unlikely)
[![docs.rs](https://docs.rs/unlikely/badge.svg)](https://docs.rs/unlikely/)
[![Build Status](https://travis-ci.org/jonas-schievink/unlikely.svg?branch=master)](https://travis-ci.org/jonas-schievink/unlikely)

Provides the `unlikely!` macro that will instruct LLVM that the contained
expression is unlikely to be executed at runtime. This is similar to the
`likely`/`unlikely` branch hints you might know from C (Rust's own versions of
these intrinsics are [not yet stable][likely-intrin]).

Please refer to the [changelog](CHANGELOG.md) to see what changed in the last
releases.

[likely-intrin]: https://github.com/rust-lang/rust/issues/26179

## Usage

Start by adding an entry to your `Cargo.toml`:

```toml
[dependencies]
unlikely = "0.1.0"
```

Then import the crate into your Rust code:

```rust
#[macro_use]
extern crate unlikely;
```

Mark code as `unlikely!`:

```rust
if self.len == self.capacity {
    unlikely!(self.reallocate());
}
```

Please also refer to the [crate documentation].

[crate documentation]: https://docs.rs/unlikely/
