use arrow::array::RecordBatch;

use super::{Attribute, columns};

pub trait AsSpanData {
    fn get_names(&self) -> impl Iterator<Item = &str>;
    fn get_svc_names(&self) -> impl Iterator<Item = String>;
}

impl AsSpanData for RecordBatch {
    fn get_names(&self) -> impl Iterator<Item = &str> {
        use arrow::array::{Array, AsArray, StringViewArray};

        struct Iter<'a> {
            arr: &'a StringViewArray,
            index: usize,
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = &'a str;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index >= self.arr.len() {
                    return None;
                }

                let item = self.arr.value(self.index);
                self.index += 1;

                Some(item)
            }
        }

        Iter {
            arr: self
                .column_by_name(columns::SPAN_NAME.name())
                .unwrap()
                .as_string_view(),
            index: 0,
        }
    }

    fn get_svc_names(&self) -> impl Iterator<Item = String> {
        use arrow::array::{Array, AsArray, ListArray};
        use arrow::datatypes::Int8Type;

        struct Iter<'a> {
            names: &'a ListArray,
            types: &'a ListArray,
            values: &'a ListArray,

            index: usize,
        }

        impl<'a> Iter<'a> {
            fn new(names: &'a ListArray, types: &'a ListArray, values: &'a ListArray) -> Self {
                assert_eq!(names.len(), types.len());
                assert_eq!(types.len(), values.len());
                Self {
                    names,
                    types,
                    values,
                    index: 0,
                }
            }
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = String;

            fn next(&mut self) -> Option<Self::Item> {
                while self.index < self.names.len() {
                    let names = self.names.value(self.index);
                    let names = names.as_string_view();

                    let types = self.types.value(self.index);
                    let types = types.as_primitive::<Int8Type>();

                    let values = self.values.value(self.index);
                    let values = values.as_binary_view();

                    for i in 0..names.len() {
                        if names.value(i) != "service.name" {
                            continue;
                        }

                        if types.value(i) != Attribute::FIELD_STR {
                            continue;
                        }

                        self.index += 1;
                        return Some(str::from_utf8(values.value(i)).unwrap().to_owned());
                    }

                    self.index += 1;
                }

                None
            }
        }

        Iter::new(
            self.column_by_name(columns::RES_ATTR_NAME.name())
                .unwrap()
                .as_list(),
            self.column_by_name(columns::RES_ATTR_TYPE.name())
                .unwrap()
                .as_list(),
            self.column_by_name(columns::RES_ATTR_VALUE.name())
                .unwrap()
                .as_list(),
        )
    }
}
