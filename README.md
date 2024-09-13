# Affirmator

A little command line utility to store affirmation and positive thoughts. Think of it is a way of giving yourself a little boost whenever you need it.

## Requirements

- [Rust](https://www.rust-lang.org/)

## Installation

Prebuilt binaries are coming soon, but for now:

```sh
# Clone the repository
git clone git@github.com:thehouseplant/affirmator

# Build locally
cargo build --release

# Run locally
./target/release/affirmator help
```

## Getting Started

```sh
# List all possible commands
affirmator help

# Add a new affirmation
affirmator add "Title" "Description"

# List all affirmations
affirmator list

# Delete an affirmation
affirmator delete [id]

# Clear all affirmations
affirmator clear
```
