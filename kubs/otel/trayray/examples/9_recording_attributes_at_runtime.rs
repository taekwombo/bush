use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use opentelemetry::KeyValue;

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .with(trayray::otel_trace_layer())
        .with(tracing_subscriber::fmt::layer())
        .init();

    record_attrs_via_tracing();
    record_attrs_via_otel();
    record_special_attrs();

    std::thread::sleep(std::time::Duration::from_millis(5_000));
}

fn record_attrs_via_tracing() {
    let span = tracing::info_span!(
        "attrs_tracing",
        attr_a = "in_scope",
        attr_b = tracing::field::Empty,
    );

    span.in_scope(|| {
        span.record("attr_b", "recorded by Span::record");
        // This will not be recorded.
        // https://docs.rs/tracing/latest/tracing/struct.Span.html#method.record
        // > Note: The fields associated with a span are part of its Metadata.
        // > The Metadata describing a particular span is constructed statically
        // > when the span is created and cannot be extended later to add new fields.
        // > Therefore, you cannot record a value for a field that was not specified
        // > when the span was created. 
        span.record("attr_c", "recorded by Span::record");

        tracing::info!("ATTRS");
    });

}

fn record_attrs_via_otel() {
    use tracing_opentelemetry::OpenTelemetrySpanExt;
    use opentelemetry::trace::TraceContextExt;

    let span = tracing::info_span!("attrs_otel");

    // Attributes recorded for Otel span, not present in tracing.

    span.in_scope(|| {
        span.set_attribute("attr_a", "OpenTelemetrySpanExt::set_attribute");
        span.context().span().set_attribute(KeyValue::new("attr_b", "SpanRef::set_attribute"));
        
        tracing::info!("ATTRS");
    });
}

fn record_special_attrs() {
    use tracing_opentelemetry::OpenTelemetrySpanExt;
    use opentelemetry::trace::TraceContextExt;

    // Special attributes according to tracing_opentelemetry must be set at span creation time.
    // https://docs.rs/tracing-opentelemetry/latest/tracing_opentelemetry/#special-fields
    //
    // But, alternative means of changing those attributes work:
    // - changing name by SpanRef::update_name
    // - changing status by SpanRef::set_status

    let span = tracing::info_span!("attrs_otel_special"/* , otel.kind = "client" */);

    // Changes span name.
    span.context().span().update_name("updated_name_non_active");

    span.in_scope(|| {
        // Sets attribute, does not change span kind.
        span.set_attribute("otel.kind", "client");
        // Sets attribute, does not change span kind.
        span.record("otel.kind", "client");

        // Changes span name.
        // span.context().span().update_name("updated_name_when_active");

        // Changes span status.
        span.context().span().set_status(opentelemetry::trace::Status::error("errored"));
        
        tracing::info!("ATTRS");
    });
}
