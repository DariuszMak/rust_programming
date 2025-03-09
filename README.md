# rust_programming
### Formatting

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

### Auditing dependencies

```sh
cargo audit --target-arch x86_64
```