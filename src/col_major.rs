use crate::{Row, Table};
use nu_protocol::{FromValue, IntoValue, Record, ShellError, Span, Type, Value};

#[derive(Debug, Clone)]
pub struct ColMajorTable {
    pub(crate) columns: Vec<String>,
    pub(crate) data: Vec<Vec<Value>>,
    pub(crate) num_rows: usize,
}

pub struct ColMajorRow<'a> {
    pub(crate) table: &'a ColMajorTable,
    pub(crate) index: usize,
}

pub struct ColMajorIter<'a> {
    pub(crate) table: &'a ColMajorTable,
    pub(crate) row: usize,
}

impl FromValue for ColMajorTable {
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

        let list = v.into_list()?;

        let num_rows = list.len();

        let mut data: Vec<Vec<Value>> = columns
            .iter()
            .map(|_| Vec::with_capacity(num_rows))
            .collect();

        for row in list {
            let record = row.into_record()?;
            for col in data.iter_mut() {
                col.push(Value::test_nothing());
            }
            for (key, val) in record {
                if let Some(pos) = columns.iter().position(|col| col == &key) {
                    *data[pos].last_mut().unwrap() = val;
                }
            }
        }

        Ok(ColMajorTable {
            columns,
            data,
            num_rows,
        })
    }
}

impl IntoValue for ColMajorTable {
    fn into_value(self, span: Span) -> Value {
        self.to_list_of_records(span)
    }
}

impl Table for ColMajorTable {
    type Iter<'a> = ColMajorIter<'a>;
    type Row<'a> = ColMajorRow<'a>;

    fn columns(&self) -> &[String] {
        &self.columns
    }

    fn get_row(&self, index: usize) -> Option<Self::Row<'_>> {
        if index < self.num_rows {
            Some(ColMajorRow { table: self, index })
        } else {
            None
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        ColMajorIter {
            table: self,
            row: 0,
        }
    }

    fn insert<F>(&mut self, name: &str, make_value: F)
    where
        Self: Sized,
        F: for<'a> FnMut(Self::Row<'a>) -> Value,
    {
        let new_values = self.iter().map(make_value).collect();
        self.columns.push(name.to_owned());
        self.data.push(new_values);
    }
}

impl<'a> Row<'a> for ColMajorRow<'a> {
    fn get_index(&self, column: usize) -> Option<&'a Value> {
        self.table.data.get(column).map(|col| &col[self.index])
    }

    fn get_named(&self, name: &str) -> Option<&'a Value> {
        self.table
            .columns
            .iter()
            .position(|col| col == name)
            .and_then(|pos| self.get_index(pos))
    }

    fn to_record(&self) -> Record {
        self.table
            .columns
            .iter()
            .zip(&self.table.data)
            .map(|(key, values)| (key.clone(), values[self.index].clone()))
            .collect()
    }
}

impl<'a> Iterator for ColMajorIter<'a> {
    type Item = ColMajorRow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.table.num_rows {
            let out = self.table.get_row(self.row);
            self.row += 1;
            out
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.table.num_rows, Some(self.table.num_rows))
    }
}
