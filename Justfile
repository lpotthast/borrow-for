# List available workflows.
list:
    just --list

# Find the minimum supported rust version.
msrv:
    cargo msrv find

# Test core-only, alloc, and std configurations, including public examples.
test toolchain="stable":
    cargo +{{toolchain}} test --locked
    cargo +{{toolchain}} test --no-default-features --locked
    cargo +{{toolchain}} test --no-default-features --features alloc --locked

# Run the same suite on the supported Rust baseline.
test-msrv:
    just test 1.85.1

# Build documentation with warnings treated as errors.
build-docs:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --no-default-features --features alloc --locked
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --no-default-features --locked

# Update dependencies, sort manifests, and format code.
tidy:
    cargo update --workspace
    cargo sort --workspace
    cargo +nightly fmt --all

# Run non-mutating validation.
verify:
    cargo +nightly fmt --all -- --check
    just test
    cargo clippy --all-targets --locked -- -D warnings -W clippy::pedantic
    cargo clippy --all-targets --no-default-features --features alloc --locked -- -D warnings -W clippy::pedantic
    cargo clippy --all-targets --no-default-features --locked -- -D warnings -W clippy::pedantic
    just build-docs
    just test-msrv
