# rust_programming

## Useful bunch of commands:

#### Useful bunch

```sh
clear ; cargo fmt --all --verbose ; cargo test ; cargo clippy -- ; cargo run ; 
cargo clean ; git add -A ; git stash save 'snapshot' ; git stash apply ; git clean -fxd ; clear ; cargo fmt --all --verbose ; cargo test ; cargo clippy -- ; cargo run ; 
cargo clean ; git add -A ; git stash save 'snapshot' ; git stash apply ; git clean -fxd ; clear ; cargo fmt --all --verbose ; cargo test ; cargo clippy -- --warn clippy::pedantic ; cargo run ; 
```

#### Create a dump to file (Linux-based environment only)

```sh
git add -A ; git stash save 'snapshot' ; git stash apply ; git clean -fxd ; tree -if --noreport | xargs -I {} sh -c '[ -f "{}" ] && echo "{}" && cat "{}" && echo' > dump.txt ; 
```

#### Update Rust

```sh
rustup update stable ; 
```

### RUN

```sh
cargo run ; 
```

#### or

```sh
cargo run main.rs ; 
```

### Run test

```sh
cargo test ; 
```

### Create a release

```sh
cargo build --release ; 
```

### Formatting

To check formatting:

```sh
cargo fmt --all --check --verbose ; 
```

To apply formatting:

```sh
cargo fmt --all --verbose ; 
```

Fix with clippy:

```sh
cargo clippy --fix --lib ; 
```

### Linting

```sh
cargo clippy -- --warn clippy::pedantic ; 
```
