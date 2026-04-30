use std::sync::Arc;
use std::sync::LazyLock;

use arrow::datatypes::{DataType, Field, Fields, Schema};
use opentelemetry_proto::tonic::common::v1::KeyValue;

pub mod columns {
    use arrow::datatypes::{DataType, Field};

    pub struct Column {
        name: &'static str,
        ty: DataType,
        nullable: bool,
        elem_nullable: bool,
        is_list: bool,
    }

    impl Column {
        const fn new(name: &'static str, ty: DataType, nullable: bool) -> Self {
            Self {
                name,
                ty,
                nullable,
                elem_nullable: false,
                is_list: false,
            }
        }

        const fn list(
            name: &'static str,
            ty: DataType,
            nullable: bool,
            elem_nullable: bool,
        ) -> Self {
            Self {
                name,
                ty,
                nullable,
                elem_nullable,
                is_list: true,
            }
        }

        pub fn name(&self) -> &'static str {
            self.name
        }

        pub fn as_list_field(&self) -> Field {
            assert!(self.is_list);

            Field::new(self.name, self.ty.clone(), self.elem_nullable)
        }

        pub fn as_field(&self) -> Field {
            if self.is_list {
                return Field::new_list(self.name, self.as_list_field(), self.nullable);
            }

            Field::new(self.name, self.ty.clone(), self.nullable)
        }
    }

    pub static TRACE_ID: Column = Column::new("trace_id", DataType::FixedSizeBinary(16), false);
    pub static SPAN_ID: Column = Column::new("span_id", DataType::Int64, false);
    pub static SPAN_NAME: Column = Column::new("span_name", DataType::Utf8View, false);
    pub static SPAN_KIND: Column = Column::new("span_kind", DataType::Int32, true);
    pub static PARENT_SPAN_ID: Column = Column::new("parent_span_id", DataType::Int64, true);
    pub static STATUS_CODE: Column = Column::new("status_code", DataType::Int32, true);
    pub static STATUS_MESSAGE: Column = Column::new("status_message", DataType::Utf8View, true);
    pub static TIME_START: Column = Column::new("time_start", DataType::UInt64, false);
    pub static TIME_END: Column = Column::new("time_end", DataType::UInt64, false);
    pub static TIME_DURATION: Column = Column::new("time_duration", DataType::UInt64, false);
    pub static RES_ATTR_NAME: Column =
        Column::list("resource_attribute_name", DataType::Utf8View, true, false);
    pub static RES_ATTR_TYPE: Column =
        Column::list("resource_attribute_type", DataType::Int8, true, false);
    pub static RES_ATTR_VALUE: Column =
        Column::list("resource_attribute_value", DataType::BinaryView, true, true);
}

pub static SCHEMA: LazyLock<Arc<Schema>> = LazyLock::new(create_schema);

fn create_schema() -> Arc<Schema> {
    use columns::*;

    let cols = vec![
        TRACE_ID.as_field(),
        SPAN_ID.as_field(),
        SPAN_NAME.as_field(),
        SPAN_KIND.as_field(),
        PARENT_SPAN_ID.as_field(),
        STATUS_CODE.as_field(),
        STATUS_MESSAGE.as_field(),
        TIME_START.as_field(),
        TIME_END.as_field(),
        TIME_DURATION.as_field(),
        RES_ATTR_NAME.as_field(),
        RES_ATTR_TYPE.as_field(),
        RES_ATTR_VALUE.as_field(),
    ];

    Arc::new(Schema::new(cols))
}

pub struct Attribute;

impl Attribute {
    pub const FIELD_NULL: i8 = 0;
    pub const FIELD_STR: i8 = 1;
    pub const FIELD_NUM_I: i8 = 2;
    pub const FIELD_NUM_F: i8 = 3;
    pub const FIELD_BOOL_T: i8 = -7;
    pub const FIELD_BOOL_F: i8 = -8;

    pub fn struct_fields() -> Fields {
        Fields::from(vec![
            Field::new("name", DataType::Utf8View, false),
            Field::new("type", DataType::Int8, false),
            Field::new("value", DataType::BinaryView, true),
        ])
    }

    pub fn data_type() -> DataType {
        DataType::Struct(Self::struct_fields())
    }

    pub fn append(
        name_builder: &mut arrow::array::ListBuilder<arrow::array::StringViewBuilder>,
        ty_builder: &mut arrow::array::ListBuilder<arrow::array::Int8Builder>,
        val_builder: &mut arrow::array::ListBuilder<arrow::array::BinaryViewBuilder>,
        attrs: &[KeyValue],
    ) {
        use opentelemetry_proto::tonic::common::v1::any_value::Value;

        for attr in attrs {
            let val = attr.value.as_ref().and_then(|v| v.value.as_ref());
            let Some(val) = val else {
                continue;
            };

            match val {
                Value::BoolValue(b) => {
                    ty_builder.values().append_value(if *b {
                        Self::FIELD_BOOL_T
                    } else {
                        Self::FIELD_BOOL_F
                    });
                    val_builder.values().append_null();
                }
                Value::IntValue(integer) => {
                    ty_builder.values().append_value(Self::FIELD_NUM_I);
                    val_builder.values().append_value(integer.to_be_bytes());
                }
                Value::DoubleValue(double) => {
                    ty_builder.values().append_value(Self::FIELD_NUM_F);
                    val_builder.values().append_value(double.to_be_bytes());
                }
                Value::StringValue(string) => {
                    ty_builder.values().append_value(Self::FIELD_STR);
                    val_builder.values().append_value(string);
                }
                _ => continue,
            }

            name_builder.values().append_value(attr.key.as_str());
        }
    }
}
