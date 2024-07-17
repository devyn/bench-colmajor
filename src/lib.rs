use nu_protocol::{FromValue, IntoValue, Record, Span, Value};

pub trait Table: FromValue + IntoValue {
    type Iter<'a>: Iterator<Item = Self::Row<'a>>
    where
        Self: 'a;

    type Row<'a>: Row<'a>
    where
        Self: 'a;

    fn columns(&self) -> &[String];
    fn get_row(&self, index: usize) -> Option<Self::Row<'_>>;
    fn iter(&self) -> Self::Iter<'_>;

    fn insert<F>(&mut self, name: &str, make_value: F)
    where
        Self: Sized,
        F: for<'a> FnMut(Self::Row<'a>) -> Value;

    fn to_list_of_records(&self, span: Span) -> Value {
        Value::list(
            self.iter()
                .map(|row| Value::record(row.to_record(), span))
                .collect(),
            span,
        )
    }
}

pub trait Row<'a> {
    fn get_index(&self, column: usize) -> Option<&'a Value>;
    fn get_named(&self, name: &str) -> Option<&'a Value>;
    fn to_record(&self) -> Record;
}

pub use col_major::*;
pub use row_major::*;

mod col_major;
mod row_major;
