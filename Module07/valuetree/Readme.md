# ValueTree

A simple and efficient key-value storage implementation in Rust using `BTreeMap`. This project provides a straightforward way to store, retrieve, and remove key-value pairs while maintaining order.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [API Reference](#api-reference)
- [Unit Tests](#unit-tests)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Ordered Storage**: Utilizes `BTreeMap` to maintain the order of keys.
- **Basic Operations**: Supports insertion, retrieval, and deletion of key-value pairs.
- **Type Safety**: Generic implementation allows for any types that implement the `Ord` trait for keys.

## Installation

To use this library, ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).


## Unit Tests

The project includes a comprehensive suite of unit tests to ensure the correctness and performance of the `DataStorage` implementation. These tests cover various aspects of the data structure's functionality:

1. **Insertion Test**: Verifies that elements can be correctly inserted and retrieved.
2. **Deletion Test**: Ensures that elements can be properly removed from the storage.
3. **Deletion of Non-existent Key**: Checks the behavior when attempting to remove a key that doesn't exist.
4. **Insertion Overwrite**: Confirms that inserting a value with an existing key overwrites the previous value.
5. **Empty Storage Test**: Validates the behavior of an empty storage structure.
6. **Performance Test**: Measures the insertion time for a large number of elements to ensure efficiency.
7. **Multiple Insertions Test**: Verifies the correctness of multiple insertions and retrievals.
8. **Removal of All Elements**: Ensures that all elements can be removed correctly.

These tests help maintain the reliability and efficiency of the `DataStorage` implementation. They can be run using the command `cargo test` in the project directory.



