all: build test

@build:
    cargo build

@test:
    cargo test --all -- --quiet

@bench:
    cargo bench

@docs: build
    cargo doc --no-deps

@format:
    cargo fmt --all -- --check

@lint:
    cargo clippy -- -D warnings
