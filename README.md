# Fibonacci Benchmark

This project benchmarks the performance of calculating Fibonacci numbers using two different approaches: **recursive** and **iterative**. The goal is to measure and compare the performance of both methods for various input values.

## Features:

- **Iterative Approach**: Efficiently calculates Fibonacci numbers by iterating through the sequence.
- **Recursive Approach**: Calculates Fibonacci numbers using recursion, which can be less efficient due to repeated calculations.
- **Benchmarking**: Using `criterion.rs` to measure the execution time of each approach and compare the results.
- **Outlier Detection**: Automatically detects and reports any outliers in the measurements.

## Fibonacci Approaches:

### Iterative Approach:

The iterative approach computes Fibonacci numbers by storing the last two numbers in the sequence and iteratively updating them. This method has better performance than recursion for large values of `n`.

```rust
fn fibonacci(n: u64) -> u64 {
    let mut a = 0; // The first Fibonacci number, F(0)
    let mut b = 1; // The second Fibonacci number, F(1)

    for _ in 0..n { // Loop n times to calculate the nth Fibonacci number
        let temp = a; // Store the current value of a (which is F(n-2))
        a = b; // Move b into a, so a becomes F(n-1)
        b = temp + b; // Calculate the next Fibonacci number (F(n-1) + F(n-2)) and store in b
    }

    a // After n iterations, a holds the nth Fibonacci number
}
```

### Recursive Approach:

The recursive approach calculates Fibonacci numbers by calling the function for n-1 and n-2 and adding them. This method can be inefficient for large values of n due to repeated calculations.

```rust
pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
```
