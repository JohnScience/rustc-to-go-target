# rustc-to-go-target

[![crates.io](https://img.shields.io/crates/v/rustc-to-go-target.svg)][`rustc-to-go-target`]
[![crates.io](https://img.shields.io/crates/d/rustc-to-go-target.svg)][`rustc-to-go-target`]

Library for conversion of targets supported by `rustc` (aka target triples) to targets supported by `go`.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
rustc-to-go-target = "0.1"
```

after that you can use it like this:

```rust
extern crate rustc_to_go_target;

fn main() {
    assert_eq!(rustc_to_go_target::convert("aarch64-apple-darwin"), "darwin/arm64");
    assert_eq!(rustc_to_go_target::convert("x86_64-unknown-freebsd"), "freebsd/amd64");
}
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`rustc-to-go-target`]: https://crates.io/crates/rustc-to-go-target
