# Singleton Instance Library for Rust

## Overview

The `singleton_instance` library is designed to make singleton pattern implementation straightforward in Rust programs. Using custom derive macros, it ensures that a given struct can have only one instance that is globally accessible. This eliminates the need for manual management of global state, making your application safer and more maintainable.

## Features

- Easy-to-use derive macro for creating singletons.
- Custom initialization of singleton instances via the `Initializable` trait.
- Thread-safe singleton initialization and retrieval.

## Installation

Add the following line to your `Cargo.toml`:

```toml
[dependencies]
singleton_instance = "0.1.0" # Use the current version
```

## Usage

First, add the library to your crate:

```rust
extern crate singleton_instance;
```

Then, implement the `Initializable` trait for your struct:

```rust
use singleton_instance::Initializable;

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

Now, you can retrieve the singleton instance as follows:

```rust
let config = AppConfig::instance();
```

## Examples

Check the `examples/` directory for more complex usage scenarios.

## Contributing

We welcome contributions from the community. Feel free to open an issue or submit a pull request!

## License

This project is licensed under the MIT License - see the `LICENSE.md` file for details.
