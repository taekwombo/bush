use std::sync::Arc;

use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use ottel_spaniel::SpanData;

// Should report number of rejected spans.
// Should be moved to bin/api
pub fn request_to_span_data(request: ExportTraceServiceRequest) -> Vec<SpanData> {
    let mut result = Vec::new();

    for rs in request.resource_spans {
        let rs_attrs = rs.resource.map(|v| v.attributes).unwrap_or(Vec::new());
        let rs_attrs = Arc::new(rs_attrs);

        for ss in rs.scope_spans {
            for span in ss.spans {
                if span.trace_id.len() != 16 {
                    continue;
                }
                if span.span_id.len() != 8 {
                    continue;
                }
                if !span.parent_span_id.is_empty() && span.parent_span_id.len() != 8 {
                    continue;
                }

                result.push(SpanData {
                    trace_id: unsafe { *span.trace_id.as_slice().as_ptr().cast() },
                    span_id: unsafe { *span.span_id.as_slice().as_ptr().cast() },
                    parent_span_id: if span.parent_span_id.len() == 8 {
                        Some(unsafe { *span.parent_span_id.as_slice().as_ptr().cast() })
                    } else {
                        None
                    },
                    name: span.name,
                    kind: span.kind,
                    status_code: span.status.as_ref().map(|s| s.code),
                    status_message: span.status.map(|s| s.message),
                    time_start: span.start_time_unix_nano,
                    time_end: span.end_time_unix_nano,
                    time_duration: span.end_time_unix_nano - span.start_time_unix_nano,

                    resource_attributes: rs_attrs.clone(),
                });
            }
        }
    }

    result
}
