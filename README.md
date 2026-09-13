# borrow-for

[![Crates.io](https://img.shields.io/crates/v/borrow-for.svg)](https://crates.io/crates/borrow-for)
[![Docs.rs](https://docs.rs/borrow-for/badge.svg)](https://docs.rs/borrow-for)
[![CI](https://github.com/lpotthast/borrow-for/actions/workflows/ci.yml/badge.svg)](https://github.com/lpotthast/borrow-for/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.85.1-blue.svg)](https://github.com/lpotthast/borrow-for/blob/main/Cargo.toml)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/borrow-for.svg)](#license)

Let generic functions accept owned values, references, and other useful forms of the same data.

Suppose you are writing an equality check. When the actual value is a `String`, you might want
callers to pass another `String`, a reference to one, or just a string literal as the expected
value. The function needs to borrow each of these in a form it can compare with the actual value.

`BorrowFor<Context>` chooses that borrowed type, called its `View`. The `Context` is a type your
function already knows, such as the actual value's type. Each input type and context together
determine one view, and `core::borrow::Borrow` provides a reference to it.

Here is why that choice helps when writing a generic comparison:

- `Expected: Borrow<Actual>` requires the expected value to borrow as `Actual`. This rules out
  a string literal when the actual value is a `String`: the literal cannot provide a `&String`.
- `Expected: Borrow<View>` lets you borrow another type, but Rust must also infer `View`.
  If several views could work, the caller needs to provide a type annotation.
- `Expected: BorrowFor<Actual>` chooses the view from the expected and actual types. The function
  can use that view without asking the caller to choose it.

`AsRef<Actual>` fixes the borrowed type in the same way as `Borrow<Actual>`. A separate generic
`AsRef<View>` can leave Rust with the same choice between several possible views.

The following `equal` function uses the same bounds as `assertr`'s `EqualTo`. One function
accepts all of these input forms:

```rust
use borrow_for::BorrowFor;
use core::borrow::Borrow;

fn equal<Actual, Expected>(actual: &Actual, expected: Expected) -> bool
where
    Actual: ?Sized + PartialEq<Expected::View>,
    Expected: BorrowFor<Actual>,
{
    let view: &Expected::View = expected.borrow();
    actual.eq(view)
}

assert!(equal(&42, 42)); // View = i32

// These string and vector examples need the alloc feature, which is enabled by default.
# #[cfg(feature = "alloc")] {
let actual = String::from("hello");
let expected = String::from("hello");
assert!(equal(&actual, &expected)); // View = String
assert!(equal(&actual, expected)); // View = String
assert!(equal(&actual, "hello")); // View = str
assert!(equal(&actual, Box::<str>::from("hello"))); // View = str

let actual = vec![String::from("hello")];
let expected = ["hello"];
assert!(equal(&actual, expected)); // Context = Vec<String>, View = [&str; 1]
assert!(equal(&actual[..], expected)); // Context = [String], View = [&str]
assert!(equal(&actual, &expected[..])); // Context = Vec<String>, View = [&str]
# }
```

Notice the last three calls. The same `expected` array is borrowed as an array when the actual
type is `Vec<String>`, and as a slice when the actual type is `[String]`. Passing a slice as the
expected value works too. The view still contains `&str` elements, which can be compared directly
with the actual `String` elements. No elements need to be cloned or converted.

The `PartialEq<Expected::View>` bound checks that the actual type can compare with the chosen
view. In the string example, `equal(&actual, expected)` takes ownership of `expected`.
Pass `&expected` if you want to keep using it after the call.

To borrow a view directly, use the `borrow_for::<Context, _>(&value)` helper.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
borrow-for = "0.1.0"
```

## Features

The crate has no third-party runtime dependencies. By default, it uses the standard library.
You can disable the default features to use it in `no_std` environments.

- `std` is enabled by default and includes `alloc`. It adds `PathBuf`/`Path` and
  `OsString`/`OsStr` support.
- For `no_std` with allocation, set `default-features = false, features = ["alloc"]`.
  This keeps support for `Box`, `Rc`, `Cow`, strings, C strings, and vectors. It also supports
  `Arc` on targets with pointer-sized atomic operations.
- For only `core`, set `default-features = false`. Values and shared or mutable references
  are still supported, as are arrays borrowed as slices.

## Custom views

A custom type can choose its own view. First, implement `Borrow<View>` to return a reference.
Then implement `BorrowFor<Context>` to tell generic code which view to use for that context.
Here, `Text` provides a `str` view when used with a `String` context:

```rust
use borrow_for::{BorrowFor, borrow_for};
use core::borrow::Borrow;

struct Text<'a>(&'a str);

impl Borrow<str> for Text<'_> {
    fn borrow(&self) -> &str {
        self.0
    }
}

impl BorrowFor<String> for Text<'_> {
    type View = str;
}

let text = Text("hello");
assert_eq!(borrow_for::<String, _>(&text), "hello");
```

[`Borrow` has a contract](https://doc.rust-lang.org/core/borrow/trait.Borrow.html): if your type
and its view implement `Eq`, `Ord`, or `Hash`, they must agree on the results. For example, a
wrapper that compares text without regard to case cannot borrow as `str`, because `str`
comparisons are case-sensitive. If borrowing a field would change how equality, ordering, or
hashing behaves, use `AsRef` or an accessor instead. This requirement relates your type to its
`View`. The `Context` only helps choose that view.

An implementation for a custom `T` does not automatically carry over to `&T` or `&mut T`.
A reference can also choose a different view from an owned value. For example, in a `str`
context, a `String` is borrowed as `str`, but an `&String` is borrowed as `String`.

## Implementation coverage

The [coverage tables](https://github.com/lpotthast/borrow-for/blob/main/COVERAGE.md)
list all supported input types and contexts. They show which `View` each combination uses,
which features you need, and why some combinations are not provided. The same tables appear
in the crate documentation. These choices of view are part of the crate's public API.

Sequences can have different element types, as the vector of `String` values and array of
string literals do in the first example. A `Vec` context also accepts boxed, reference-counted,
and `Cow` slices. A `Cow<str>` context accepts owned strings and borrowed forms, while a
`Cow<[T]>` context accepts slices and vectors. These implementations borrow the existing
elements. The function using the view supplies any comparison bounds it needs.

## Development

Run `just --list` to see the available development commands. `just verify` checks formatting,
runs tests and Clippy, builds the documentation, and checks the minimum supported Rust version.
It covers the core-only, `alloc`, and `std` configurations. The integration tests in
`tests/borrowing.rs` have no third-party dependencies.

Rust 1.85.1 is the oldest supported version. `just test-msrv` runs the tests and documentation
examples with that toolchain. `just msrv` runs `cargo msrv find` to find the minimum version
that can build the crate.

## License

Licensed under either [Apache License, Version 2.0](https://github.com/lpotthast/borrow-for/blob/main/LICENSE-APACHE)
or [MIT license](https://github.com/lpotthast/borrow-for/blob/main/LICENSE-MIT),
at your option.
