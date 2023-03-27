# build-alert

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/build--alert-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/build-alert)
[<img alt="crates.io" src="https://img.shields.io/crates/v/build-alert.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/build-alert)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-build--alert-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/build-alert)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/build-alert/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/build-alert/actions?query=branch%3Amaster)

Display a message in the Cargo build output during compilation.

```toml
[dependencies]
build-alert = "0.1"
```

<br>

## Example

```rust
#[cfg(debug_assertions)]
build_alert::yellow! {"
NOTE:  use --release
  Syn's test suite has some tests that run on every source file
  and test case in the rust-lang/rust repo, which can be pretty
  slow in debug mode. Consider running cargo test with `--release`
  to speed things up.
"}

#[cfg(not(feature = "all-features"))]
build_alert::red! {"
ERROR:  use --all-features
  Syn's test suite normally only works with all-features enabled.
  Run again with `--all-features`, or run with `--features test`
  to bypass this check.
"}
```

![screenshot](https://user-images.githubusercontent.com/1940490/227811885-3eca7b65-0425-4be5-aa1a-cf52d8014817.png)

<br>

#### License

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
