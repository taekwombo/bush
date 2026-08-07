//! https://docs.rs/opentelemetry_sdk/latest/opentelemetry_sdk/metrics/struct.MeterProviderBuilder.html#method.with_periodic_exporter
//! Set OTEL_METRIC_EXPORT_INTERVAL in ms to configure export interval.
fn main() {
    trayray::otel_metrics_setup();

    let meter = opentelemetry::global::meter("trayray-example");
    let counter = meter.u64_counter("example_ticks")
        .with_description("counting thread sleeps")
        .build();

    let args = format!("{:?}", std::env::args().collect::<Vec<_>>());
    let labels = [opentelemetry::KeyValue::new("args", args)];
    let mut buffer = String::new();

    println!(">");
    while let Ok(n) = std::io::stdin().read_line(&mut buffer) {
        counter.add(n as u64, &labels);
        println!(">");
    }
}
