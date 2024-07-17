use crate::{Row, Table};
use nu_protocol::{FromValue, IntoValue, Record, ShellError, Span, Type, Value};

#[derive(Debug, Clone)]
pub struct RowMajorTable {
    pub(crate) columns: Vec<String>,
    pub(crate) data: Vec<Vec<Value>>,
}

pub struct RowMajorRow<'a> {
    pub(crate) columns: &'a Vec<String>,
    pub(crate) row: &'a [Value],
}

pub struct RowMajorIter<'a> {
    pub(crate) columns: &'a Vec<String>,
    pub(crate) rows: std::slice::Iter<'a, Vec<Value>>,
}

impl FromValue for RowMajorTable {
    fn from_value(v: Value) -> Result<Self, ShellError> {
        let Type::Table(column_types) = v.get_type() else {
            return Err(ShellError::TypeMismatch {
                err_message: "expected table".into(),
                span: v.span(),
            });
        };
        let columns: Vec<String> = column_types
            .iter()
            .map(|(column, _)| column.to_owned())
            .collect();

        let data = v
            .into_list()?
            .into_iter()
            .map(|row| {
                let record = row.into_record()?;
                let mut row = vec![Value::test_nothing(); columns.len()];
                for (key, val) in record {
                    if let Some(pos) = columns.iter().position(|col| col == &key) {
                        row[pos] = val;
                    }
                }
                Ok(row)
            })
            .collect::<Result<Vec<_>, ShellError>>()?;
        Ok(RowMajorTable { columns, data })
    }
}

impl IntoValue for RowMajorTable {
    fn into_value(self, span: Span) -> Value {
        self.to_list_of_records(span)
    }
}

impl Table for RowMajorTable {
    type Iter<'a> = RowMajorIter<'a>;
    type Row<'a> = RowMajorRow<'a>;

    fn columns(&self) -> &[String] {
        &self.columns
    }

    fn get_row(&self, index: usize) -> Option<Self::Row<'_>> {
        self.data.get(index).map(|row| RowMajorRow {
            columns: &self.columns,
            row,
        })
    }

    fn iter(&self) -> Self::Iter<'_> {
        RowMajorIter {
            columns: &self.columns,
            rows: self.data.iter(),
        }
    }

    fn insert<F>(&mut self, name: &str, mut make_value: F)
    where
        Self: Sized,
        F: for<'a> FnMut(Self::Row<'a>) -> Value,
    {
        self.columns.push(name.to_owned());
        for row in self.data.iter_mut() {
            let new_value = make_value(RowMajorRow {
                columns: &self.columns,
                row: &row,
            });
            row.push(new_value);
        }
    }
}

impl<'a> Row<'a> for RowMajorRow<'a> {
    fn get_index(&self, column: usize) -> Option<&'a Value> {
        self.row.get(column)
    }

    fn get_named(&self, name: &str) -> Option<&'a Value> {
        self.columns
            .iter()
            .position(|col| col == name)
            .and_then(|pos| self.row.get(pos))
    }

    fn to_record(&self) -> Record {
        self.columns
            .iter()
            .zip(self.row)
            .map(|(key, val)| (key.clone(), val.clone()))
            .collect()
    }
}

impl<'a> Iterator for RowMajorIter<'a> {
    type Item = RowMajorRow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().map(|row| RowMajorRow {
            columns: self.columns,
            row,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.rows.size_hint()
    }
}
