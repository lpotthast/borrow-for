# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.1.0] - 2026-09-13

### Added

- `BorrowFor<Context>` selects which type to borrow from a value, and `borrow_for` returns the reference.
- Built-in implementations for values, references, arrays, strings, slices and smart pointers.
- No runtime dependencies. The default `std` feature includes `alloc`; disabling default features allows `no_std` use.
- Requires Rust 1.85.1 or later.
