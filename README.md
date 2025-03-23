# rust_programming

## Useful bunch of commands:

```sh
cargo clean ; git clean -fxd ; clear ; cargo fmt --all --verbose ; cargo test ; cargo clippy -- ; cargo run ; 
cargo clean ; git clean -fxd ; clear ; cargo fmt --all --verbose ; cargo test ; cargo clippy -- --warn clippy::pedantic ; cargo run ; 
```

### RUN

```sh
cargo run
```

#### or

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

Fix with clippy:

```sh
cargo clippy --fix --lib
```

### Linting

```sh
cargo clippy -- --warn clippy::pedantic
```
