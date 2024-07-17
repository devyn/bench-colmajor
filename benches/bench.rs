#![feature(test)]

extern crate test;

use std::hint::black_box;
use test::bench::Bencher;

#[bench]
fn row_major(b: &mut Bencher) {
    let rows: Vec<Vec<i32>> = (0..10000).map(|_| (0..10).collect()).collect();
    b.iter(|| {
        let mut row_sums = vec![0; 10];
        for row in &rows {
            for (sum, val) in row_sums.iter_mut().zip(row) {
                *sum += *val;
            }
        }
        black_box(row_sums)
    });
}

#[bench]
fn col_major(b: &mut Bencher) {
    let cols: Vec<Vec<i32>> = (0..10).map(|_| (0..10000).collect()).collect();
    b.iter(|| {
        black_box(
            cols.iter()
                .map(|vals| vals.iter().copied().sum::<i32>())
                .collect::<Vec<i32>>(),
        )
    });
}
