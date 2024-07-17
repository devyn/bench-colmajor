#![feature(test)]

extern crate test;

use bench_colmajor::{ColMajorTable, Row, RowMajorTable, Table};
use mimalloc::MiMalloc;
use nu_protocol::{record, FromValue, IntoValue, ShellError, Span, Value};
use std::hint::black_box;
use test::bench::Bencher;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn example_table() -> Value {
    Value::test_list(
        (0..10000)
            .map(|index: i64| {
                Value::test_record(record! {
                    "index" => Value::test_int(index),
                    "dec" => Value::test_string(index.to_string()),
                    "hex" => Value::test_string(format!("{:08x}", index)),
                    "bytes" => Value::test_binary(index.to_ne_bytes()),
                })
            })
            .collect::<Vec<_>>(),
    )
}

#[bench]
fn row_major_from_value(b: &mut Bencher) {
    let table = example_table();
    b.iter(|| black_box(RowMajorTable::from_value(table.clone()).unwrap()))
}

#[bench]
fn col_major_from_value(b: &mut Bencher) {
    let table = example_table();
    b.iter(|| black_box(ColMajorTable::from_value(table.clone()).unwrap()))
}

#[bench]
fn row_major_get_row_as_record(b: &mut Bencher) {
    let table = RowMajorTable::from_value(example_table()).unwrap();
    assert_eq!(
        table.get_row(5000).unwrap().get_named("index"),
        Some(&Value::test_int(5000))
    );
    b.iter(|| black_box(table.get_row(5000).unwrap().to_record()))
}

#[bench]
fn col_major_get_row_as_record(b: &mut Bencher) {
    let table = ColMajorTable::from_value(example_table()).unwrap();
    assert_eq!(
        table.get_row(5000).unwrap().get_named("index"),
        Some(&Value::test_int(5000))
    );
    b.iter(|| black_box(table.get_row(5000).unwrap().to_record()))
}

#[bench]
fn row_major_into_value(b: &mut Bencher) {
    let table = RowMajorTable::from_value(example_table()).unwrap();
    b.iter(|| black_box(table.clone().into_value(Span::test_data())))
}

#[bench]
fn col_major_into_value(b: &mut Bencher) {
    let table = ColMajorTable::from_value(example_table()).unwrap();
    b.iter(|| black_box(table.clone().into_value(Span::test_data())))
}

fn sum(table: &impl Table, column: &str) -> Result<Option<Value>, ShellError> {
    let mut iter = table
        .iter()
        .map(|row| row.get_named(column).expect("not found"));

    let Some(mut acc) = iter.next().cloned() else {
        return Ok(None);
    };

    for value in iter {
        acc = acc.add(Span::test_data(), value, Span::test_data())?;
    }

    Ok(Some(acc))
}

#[bench]
fn row_major_sum_column(b: &mut Bencher) {
    let table = RowMajorTable::from_value(example_table()).unwrap();
    assert!(sum(&table, "index").unwrap().unwrap().as_int().unwrap() > 0);
    b.iter(|| black_box(sum(&table, "index")))
}

#[bench]
fn col_major_sum_column(b: &mut Bencher) {
    let table = ColMajorTable::from_value(example_table()).unwrap();
    assert!(sum(&table, "index").unwrap().unwrap().as_int().unwrap() > 0);
    b.iter(|| black_box(sum(&table, "index")))
}

fn index_squared<'a>(row: impl Row<'a>) -> Value {
    let value = &row.get_named("index").expect("not found");
    let index = value.as_int().unwrap();
    Value::test_int(index * index)
}

#[bench]
fn row_major_insert_column(b: &mut Bencher) {
    let table = RowMajorTable::from_value(example_table()).unwrap();
    {
        let mut table = table.clone();
        table.insert("index_squared", |row| index_squared(row));
        assert_eq!(
            table.get_row(2).unwrap().get_named("index_squared"),
            Some(&Value::test_int(4))
        );
    }
    b.iter(|| {
        let mut table = table.clone();
        black_box(table.insert("index_squared", |row| index_squared(row)))
    })
}

#[bench]
fn col_major_insert_column(b: &mut Bencher) {
    let table = ColMajorTable::from_value(example_table()).unwrap();
    {
        let mut table = table.clone();
        table.insert("index_squared", |row| index_squared(row));
        assert_eq!(
            table.get_row(2).unwrap().get_named("index_squared"),
            Some(&Value::test_int(4))
        );
    }
    b.iter(|| {
        let mut table = table.clone();
        black_box(table.insert("index_squared", |row| index_squared(row)))
    })
}
