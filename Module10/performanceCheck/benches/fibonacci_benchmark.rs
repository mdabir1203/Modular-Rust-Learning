#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use std::sync::RwLock;

pub fn fib_recursive(n: u64) -> u64 {
   match n {
       0 => 1,
       1 => 1,
       n => fib_recursive(n - 1) + fib_recursive(n - 2),
   }
   }

// Iterative Fibonacci
pub fn fib_iterative(n: u64) -> u64 {
   let mut a = 1;
   let mut b = 1;
   for _ in 2..=n {
       let temp = a + b;
       a = b;
       b = temp;
   }
   b
}
// Memoized Fibonacci
lazy_static! {
   static ref FIB_CACHE: RwLock<Vec<u64>> = RwLock::new(vec![1, 1]);
}

// Matrix Exponentiation Fibonacci
pub fn fib_matrix(n: u64) -> u64 {
   if n == 0 {
       return 1;
   }
   let mut m = ((1, 1), (1, 0));
   let mut result = ((1, 0), (0, 1)); // Identity matrix
   let mut power = n - 1;
   while power > 0 {
       if power % 2 == 1 {
           result = multiply(result, m);
       }
       m = multiply(m, m);
       power /= 2;
   }
   result.0 .0
}

fn multiply(a: ((u64, u64), (u64, u64)), b: ((u64, u64), (u64, u64))) -> ((u64, u64), (u64, u64)) {
   (
       (a.0 .0 * b.0 .0 + a.0 .1 * b.1 .0, a.0 .0 * b.0 .1 + a.0 .1 * b.1 .1),
       (a.1 .0 * b.0 .0 + a.1 .1 * b.1 .0, a.1 .0 * b.0 .1 + a.1 .1 * b.1 .1),
   )
}

fn fibonacci(n: u64) -> u64 {
   let mut a = 0;
   let mut b = 1;
    match n {
       0 => b,
       _ => {
           for _ in 0..n {
               let c = a + b;
               a = b;
               b = c;
           }
           b
       }
   }
}
// Benchmarking setup
fn bench_fib(c: &mut Criterion) {
   c.bench_function("fib_recursive 20", |b| b.iter(|| fib_recursive(black_box(10))));
   c.bench_function("fib_iterative 20", |b| b.iter(|| fib_iterative(black_box(10))));
   c.bench_function("fib_matrix 20", |b| b.iter(|| fib_matrix(black_box(10))));
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);