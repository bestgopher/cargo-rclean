# cargo-rclean
Execute `cargo clean` for all cargo projects in the directory.

# install
```shell
➜ cargo install --git https://github.com/bestgopher/cargo-rclean
```

# usage
```shell
➜ cargo rclean
```

# Examples
There are three cargo projects in current working directory：
```shell
➜ tree -L 2
.
├── rclean_test_1
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   └── target
├── rclean_test_2
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   └── target
└── rclean_test_3
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    └── target
```
Then I execute: `cargo rclean` :
```shell
➜ tree -L 2   
.
├── rclean_test_1
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── rclean_test_2
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
└── rclean_test_3
    ├── Cargo.lock
    ├── Cargo.toml
    └── src

```
All targets have been removed.
