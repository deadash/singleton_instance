# Singleton Instance Library for Rust

## Overview

The `singleton_instance` library is designed to make singleton pattern implementation straightforward in Rust programs. Using custom derive macros, it ensures that a given struct can have only one instance that is globally accessible. This eliminates the need for manual management of global state, making your application safer and more maintainable.

## Features

- Easy-to-use derive macro for creating singletons.
- Custom initialization of singleton instances via the `Initializable` trait.
- Thread-safe singleton initialization and retrieval.

## Installation

To add this library to your project, insert the following lines into your `Cargo.toml`:

```toml
[dependencies]
singleton_instance = { git = "https://github.com/deadash/singleton_instance", package = "singleton_instance" }
singleton_macro = { git = "https://github.com/deadash/singleton_instance", package = "singleton_macro" }
```

## Usage

First, add the necessary imports:

```rust
use singleton_instance::Initializable;
use singleton_macro::Instance;
```

Next, define your struct and implement the `Initializable` trait:

```rust
#[derive(Instance)]
struct AppConfig {
    db_url: String,
    port: u16,
}

impl Initializable for AppConfig {
    fn initialize() -> Self {
        AppConfig {
            db_url: "localhost".to_string(),
            port: 8080,
        }
    }
}
```

Retrieve the singleton instance with:

```rust
let config = AppConfig::instance();
```

## Examples

Check the `examples/` directory for more complex usage scenarios.

## Contributing

We welcome contributions from the community. Feel free to open an issue or submit a pull request!

## License

This project is licensed under the MIT License - see the `LICENSE.md` file for details.
