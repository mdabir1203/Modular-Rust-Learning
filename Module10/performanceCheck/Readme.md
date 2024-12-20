# Performance Benchmarking with Criterion

This project benchmarks various Fibonacci implementations and sorting algorithms using the Criterion library in Rust. The goal is to evaluate the performance of different algorithms and provide insights into their efficiency.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
- [Benchmarking](#benchmarking)
- [Fibonacci Implementations](#fibonacci-implementations)
- [Sorting Algorithms](#sorting-algorithms)
- [Dependencies](#dependencies)
- [License](#license)

## Features

- Benchmarking of multiple Fibonacci algorithms:
  - Recursive
  - Iterative
  - Memoized
  - Matrix Exponentiation
- Benchmarking of sorting algorithms:
  - Bubble Sort
  - Quick Sort
- Random data generation for testing

## Getting Started

To get started with this project, follow these steps:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/yourproject.git
   cd yourproject
   ```

2. **Install Rust**: If you haven't already, install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

3. **Add dependencies**: Ensure that the following dependencies are included in your `Cargo.toml` file:
   ```toml
   [dependencies]
   criterion = "0.3"
   lazy_static = "1.4"
   rand = "0.8"  # or the latest version
   ```

## Benchmarking

To run the benchmarks, use the following command:

   ```bash
   cargo bench
   ```

This command will execute the benchmarks defined in the `benches` directory and provide detailed output on the performance of each algorithm.

## Fibonacci Implementations

The project includes several implementations of the Fibonacci sequence:

- **Recursive**: A straightforward recursive approach.
- **Iterative**: An iterative method that uses a loop.
- **Memoized**: A version that caches results to improve performance.
- **Matrix Exponentiation**: A mathematical approach using matrix multiplication.

## Sorting Algorithms

The project benchmarks the following sorting algorithms:

- **Bubble Sort**: A simple comparison-based sorting algorithm.
- **Quick Sort**: A more efficient, divide-and-conquer sorting algorithm.

## Dependencies

This project relies on the following Rust crates:

- `criterion`: For benchmarking.
- `lazy_static`: For creating static variables that require initialization.
- `rand`: For generating random data for testing.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
