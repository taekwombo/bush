use arrow::array::*;
use arrow::datatypes::*;

use super::{Attribute, columns};

pub trait AsSpanData {
    fn get_names(&self) -> impl Iterator<Item = &str>;
    fn get_svc_names(&self) -> impl Iterator<Item = String>;
    fn get_spans(&self) -> impl Iterator<Item = Span>;
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

    fn get_spans(&self) -> impl Iterator<Item = Span> {
        use super::columns::*;

        let trace_id = self
            .column_by_name(TRACE_ID.name())
            .unwrap()
            .as_fixed_size_binary();
        let span_id = self
            .column_by_name(SPAN_ID.name())
            .unwrap()
            .as_primitive::<Int64Type>();
        let span_name = self
            .column_by_name(SPAN_NAME.name())
            .unwrap()
            .as_string_view();
        let span_kind = self
            .column_by_name(SPAN_KIND.name())
            .unwrap()
            .as_primitive::<Int32Type>();
        let parent_span_id = self
            .column_by_name(PARENT_SPAN_ID.name())
            .unwrap()
            .as_primitive::<Int64Type>();
        let status_code = self
            .column_by_name(STATUS_CODE.name())
            .unwrap()
            .as_primitive::<Int32Type>();
        let status_message = self
            .column_by_name(STATUS_MESSAGE.name())
            .unwrap()
            .as_string_view();

        let time_start = self
            .column_by_name(TIME_START.name())
            .unwrap()
            .as_primitive::<UInt64Type>();
        let time_end = self
            .column_by_name(TIME_END.name())
            .unwrap()
            .as_primitive::<UInt64Type>();
        let time_duration = self
            .column_by_name(TIME_DURATION.name())
            .unwrap()
            .as_primitive::<UInt64Type>();

        let res_attr_name: &GenericListArray<i32> =
            self.column_by_name(RES_ATTR_NAME.name()).unwrap().as_list();
        let res_attr_type: &GenericListArray<i32> =
            self.column_by_name(RES_ATTR_TYPE.name()).unwrap().as_list();
        let res_attr_values: &GenericListArray<i32> = self
            .column_by_name(RES_ATTR_VALUE.name())
            .unwrap()
            .as_list();

        let span_attr_name: &GenericListArray<i32> = self
            .column_by_name(SPAN_ATTR_NAME.name())
            .unwrap()
            .as_list();
        let span_attr_type: &GenericListArray<i32> = self
            .column_by_name(SPAN_ATTR_TYPE.name())
            .unwrap()
            .as_list();
        let span_attr_values: &GenericListArray<i32> = self
            .column_by_name(SPAN_ATTR_VALUE.name())
            .unwrap()
            .as_list();

        trace_id
            .iter()
            .enumerate()
            .map(move |(idx, trace_id)| Span {
                trace_id: read_hex::<{ 16 * 2 }>(trace_id.unwrap()),
                span_id: read_hex::<{ 8 * 2 }>(span_id.value(idx).to_be_bytes().as_slice()),
                parent_span_id: if parent_span_id.is_null(idx) {
                    None
                } else {
                    Some(read_hex::<{ 8 * 2 }>(
                        parent_span_id.value(idx).to_be_bytes().as_slice(),
                    ))
                },
                name: span_name.value(idx).to_owned(),
                kind: if span_kind.is_null(idx) {
                    None
                } else {
                    Some(span_kind.value(idx))
                },
                status: if status_code.is_null(idx) {
                    None
                } else {
                    Some(Status {
                        code: status_code.value(idx),
                        message: if status_message.is_null(idx) {
                            None
                        } else {
                            Some(status_message.value(idx).to_owned())
                        },
                    })
                },
                time: Time {
                    start_ms: time_start.value(idx) / 1_000_000,
                    end_ms: time_end.value(idx) / 1_000_000,
                    duration_ms: time_duration.value(idx) / 1_000_000,
                },
                resource_attributes: if res_attr_name.is_null(idx) {
                    None
                } else {
                    Some(Attributes::new(
                        res_attr_name.value(idx).as_string_view(),
                        res_attr_type.value(idx).as_primitive::<Int8Type>(),
                        res_attr_values.value(idx).as_binary_view(),
                    ))
                },
                attributes: Attributes::new(
                    span_attr_name.value(idx).as_string_view(),
                    span_attr_type.value(idx).as_primitive::<Int8Type>(),
                    span_attr_values.value(idx).as_binary_view(),
                ),
            })
    }
}

fn read_hex<const SIZE: usize>(value: &[u8]) -> String {
    let mut buf: [u8; SIZE] = [0; SIZE];
    const_hex::encode_to_str(value, &mut buf).unwrap();

    String::from_utf8(buf.to_vec()).unwrap()
}

// TODO: should be shared.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Span {
    trace_id: String,
    span_id: String,
    parent_span_id: Option<String>,
    name: String,
    kind: Option<i32>,
    status: Option<Status>,
    time: Time,
    attributes: Attributes,
    resource_attributes: Option<Attributes>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    code: i32,
    message: Option<String>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    start_ms: u64,
    end_ms: u64,
    duration_ms: u64,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    keys: Vec<String>,
    values: Vec<opentelemetry_proto::tonic::common::v1::any_value::Value>,
}

impl Attributes {
    fn new(
        names: &StringViewArray,
        types: &PrimitiveArray<Int8Type>,
        values: &BinaryViewArray,
    ) -> Self {
        use opentelemetry_proto::tonic::common::v1::any_value::Value;

        let keys = names.iter().map(|v| String::from(v.unwrap())).collect();
        let values = types
            .iter()
            .enumerate()
            .map(|(idx, t)| match t.unwrap() {
                Attribute::FIELD_BOOL_T => Value::BoolValue(true),
                Attribute::FIELD_BOOL_F => Value::BoolValue(false),
                Attribute::FIELD_STR => {
                    Value::StringValue(String::from_utf8(values.value(idx).to_owned()).unwrap())
                }
                Attribute::FIELD_NUM_I => Value::IntValue(i64::from_be_bytes(unsafe {
                    *(values.value(idx) as *const _ as *const [u8; 8])
                })),
                Attribute::FIELD_NUM_F => Value::DoubleValue(f64::from_be_bytes(unsafe {
                    *(values.value(idx) as *const _ as *const [u8; 8])
                })),
                _ => unreachable!(),
            })
            .collect();

        Self { keys, values }
    }
}
