use std::sync::Arc;

use arrow::array::RecordBatch;
use parquet::arrow::ArrowSchemaConverter;
use parquet::arrow::arrow_reader::{ParquetRecordBatchReaderBuilder, RowFilter};

use ottel_spaniel::arrow::Filter;

fn read_arrow() {
    use ottel_spaniel::arrow::columns::*;

    let time = std::time::Instant::now();
    let path = "data-arrow/spaniel-live-arrow-0";

    let file = std::fs::File::open(path).unwrap();
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
    // let schema = builder.parquet_schema();
    let schema = ArrowSchemaConverter::new()
        .convert(&ottel_spaniel::arrow::SCHEMA)
        .unwrap();

    // Find time.start column stats
    let metadata = builder.metadata();
    for rg in metadata.row_groups() {
        for col in rg.columns() {
            if col.column_descr().name() != TIME_START.name() {
                continue;
            }

            println!("{:#?}", col.statistics());
        }
    }

    let st = Filter::new_str(&schema, SPAN_NAME.name(), "d").starts_with();
    let cn = Filter::new_str(&schema, SPAN_NAME.name(), "org").contains();
    let eq = ottel_spaniel::arrow::Boolean::and(vec![Arc::new(st), Arc::new(cn)]);

    let projection =
        parquet::arrow::ProjectionMask::columns(&schema, [SPAN_NAME.name(), TRACE_ID.name()]);

    let reader = builder
        .with_projection(projection)
        .with_row_filter(RowFilter::new(vec![Box::new(eq)]))
        .build()
        .unwrap();

    for batch in reader {
        convert(&batch.unwrap());
    }

    println!("Elapsed: {}", time.elapsed().as_millis());
}

fn convert(batch: &RecordBatch) {
    use arrow::array::AsArray;
    use ottel_spaniel::arrow::columns::*;

    let trace_id = &batch
        .column_by_name(TRACE_ID.name())
        .unwrap()
        .as_fixed_size_binary();
    let name = &batch.column_by_name(SPAN_NAME.name()).unwrap();
    let name = name.as_string_view();

    for i in 0..batch.num_rows() {
        let trace_id = trace_id.value(i);
        let mut bytes: [u8; 16 * 2] = [0; 16 * 2];
        println!(
            "{} {} {}",
            i,
            name.value(i),
            trace_id_as_hex(trace_id, &mut bytes)
        );
    }
}

fn trace_id_as_hex<'a>(value: &'a [u8], bytes: &'a mut [u8; 32]) -> &'a str {
    const_hex::encode_to_str(value, bytes).unwrap();

    unsafe { std::str::from_utf8_unchecked(bytes) }
}

async fn read_vortex() {
    use vortex::VortexSessionDefault;
    use vortex::expr::*;
    use vortex::io::runtime::current::*;
    use vortex::io::runtime::*;
    use vortex::io::session::RuntimeSessionExt;
    use vortex::session::*;

    use ottel_spaniel::Format;
    use ottel_spaniel::vortex::read::*;

    let rt = CurrentThreadRuntime::new();
    let session = VortexSession::default().with_handle(rt.handle());

    let format = Format::Vortex {
        session,
        runtime: rt,
    };
    let files = ottel_spaniel::misc::read_dir("data-vortex")
        .map(|v| std::path::PathBuf::from(format!("data-vortex/{v}")).into())
        .collect();

    let mut read = Read::new(&format, files).with_projection(select(["name"], root()));

    let mut unique = std::collections::HashSet::new();
    let time = std::time::Instant::now();

    while let Some(arr) = read.next_batch().await {
        for name in arr.get_names() {
            let name = name.as_utf8().value().unwrap().as_str();

            if unique.contains(name) {
                continue;
            }

            unique.insert(name.to_owned());
        }
    }
    println!("{:#?}", unique);
    println!("Elapsed: {}", time.elapsed().as_millis());
}

#[tokio::main]
async fn main() {
    read_arrow();
    // if false {
    read_vortex().await;
    // }
}
