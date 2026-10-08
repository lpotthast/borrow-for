# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.1.2] - 2026-10-08

### Added

- Arrays of one owned and borrowed family borrow for each other's contexts when they have the same length: `[&str; N]`
  in a `[String; N]` context and `[String; N]` in a `[&str; N]` context, including references to these arrays.
- Slice references of one family borrow for each other's contexts: `&[&str]` in an `&[String]` context and `&[String]`
  in an `&[&str]` context.

## [0.1.1] - 2026-10-08

### Added

- Vectors and slices of one owned and borrowed family borrow for each other's contexts: `Vec<&str>` and `[&str]` in
  `Vec<String>` and `[String]` contexts, and `Vec<String>` and `[String]` in `Vec<&str>` and `[&str]` contexts. The
  same views exist for `CString`/`CStr`, `PathBuf`/`Path`, and `OsString`/`OsStr`. Comparing these sequences needs
  `PartialEq` between the element types. `String`/`&str`, `PathBuf`/`&Path`, and `OsString`/`&OsStr` sequences
  compare in both directions. `CString` and `&CStr` elements do not compare on Rust 1.85.1, and Rust 1.90 and later
  only compare `CString` with `&CStr`, not `&CStr` with `CString`.

## [0.1.0] - 2026-09-13

### Added

- `BorrowFor<Context>` selects which type to borrow from a value, and `borrow_for` returns the reference.
- Built-in implementations for values, references, arrays, strings, slices and smart pointers.
- No runtime dependencies. The default `std` feature includes `alloc`; disabling default features allows `no_std` use.
- Requires Rust 1.85.1 or later.

[Unreleased]: https://github.com/lpotthast/borrow-for/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/lpotthast/borrow-for/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/lpotthast/borrow-for/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/lpotthast/borrow-for/releases/tag/v0.1.0
