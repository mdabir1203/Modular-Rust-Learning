#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
// Real life algorithms

fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = arr.len() / 2;
    let (left, right) = arr.split_at_mut(pivot);
    quick_sort(left);
    quick_sort(right);
}


fn generate_large_data(size: usize) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| rng.gen_range(0..1000)).collect()
    }

// Benchmark setup
fn bench_sorting(c: &mut Criterion) {
    let mut arr1 = vec![5, 2, 9, 1, 5, 6, 3, 8, 7, 4];
    let mut arr2 = vec![5, 2, 9, 1, 5, 6, 3, 8, 7, 4,3,2,1, -1, 100 , -200, 2, 34, 45 , 71];;

    c.bench_function("bubble_sort", |b| b.iter(|| bubble_sort(&mut arr1)));
    c.bench_function("quick_sort", |b| {
        let mut data = generate_large_data(10); // Ensure this generates a sufficiently large dataset
        b.iter(|| quick_sort(black_box(&mut data)));
    });
}


criterion_group!(benches, bench_sorting);
criterion_main!(benches);