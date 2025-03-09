# rust_programming
### RUN

```sh
cargo run main.rs
```

### Run test

```sh
cargo test
```

### Formatting

To check formatting:

```sh
cargo fmt --all --check --verbose
```

To apply formatting:

```sh
cargo fmt --all --verbose
```

### Linting

```sh
cargo clippy -- --warn clippy::pedantic
```

### Useful bunch of commands:

```sh
cargo fmt --all --verbose ; cargo test ; cargo clippy -- --warn clippy::pedantic
```