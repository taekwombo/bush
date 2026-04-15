use arrow::array::RecordBatch;
use parquet::arrow::arrow_reader::{RowFilter, ParquetRecordBatchReaderBuilder};

use ottel_spaniel::arrow::Filter;

fn read_arrow() {
    let path = "data-arrow/spaniel-live-arrow";
    let span_name = "d";

    let file = std::fs::File::open(path).unwrap();
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
    let schema = builder.parquet_schema();

    let mut eq = Filter::new_str(&schema, "name", span_name);
    eq.starts_with();

    let projection = parquet::arrow::ProjectionMask::columns(&schema, [
        "name",
        "trace_id",
    ]);

    let mut reader = builder
        .with_projection(projection)
        .with_row_filter(RowFilter::new(vec![
            Box::new(eq),
        ]))
        .build()
        .unwrap();

    let time = std::time::Instant::now();
    while let Some(batch) = reader.next() {
        convert(&batch.unwrap());
    }
    println!("Elapsed: {}", time.elapsed().as_nanos());
}

fn convert(batch: &RecordBatch) {
    use arrow::datatypes::*;
    use arrow::array::AsArray;

    let trace_id = &batch.column_by_name("trace_id").unwrap().as_fixed_size_list();
    let name = &batch.column_by_name("name").unwrap();
    let name = name.as_string_view();

    for i in 0..batch.num_rows() {
        let trace_id = trace_id.value(i);
        let trace_id = trace_id.as_primitive::<UInt8Type>();
        let mut bytes: [u8; 16 * 2] = [0; 16 * 2];
        println!("{} {} {}", i, name.value(i), trace_id_as_hex(&trace_id, &mut bytes));
    }
}

fn trace_id_as_hex<'a>(
    value: &'a arrow::array::PrimitiveArray<arrow::datatypes::UInt8Type>,
    bytes: &'a mut [u8; 32],
) -> &'a str {
        let value = value.values().inner().as_slice();
        const_hex::encode_to_str(value, bytes).unwrap();

        unsafe { std::str::from_utf8_unchecked(bytes) }
}

async fn read_vortex() {
    use vortex::VortexSessionDefault;
    use vortex::array::arrays::Struct;
    use vortex::session::*;
    use vortex::io::runtime::*;
    use vortex::io::runtime::current::*;
    use vortex::io::session::RuntimeSessionExt;
    use vortex::file::OpenOptionsSessionExt;

    let path = "data-vortex/spaniel-live-vortex-0";
    let rt = CurrentThreadRuntime::new();
    let session = VortexSession::default().with_handle(rt.handle());
    let oo = session.open_options().open_path(path).await.expect("ok");

    // println!("{:#?}", oo.dtype());

    for i in oo.scan().unwrap().into_iter(&rt).unwrap() {
        let i = i.unwrap();
        let s = i.as_struct_typed();
        println!("{:?}", s.names());

        let st = i.downcast::<Struct>();
        for i in 0..st.len() {
            println!("{:#?}", st.scalar_at(i));
        }
    }
}

#[tokio::main]
async fn main() {
    if false {
        read_arrow();
    }
    read_vortex().await;
}
