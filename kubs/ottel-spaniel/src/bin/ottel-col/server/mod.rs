use std::time::Duration;

use poem::listener::TcpListener;
use poem::middleware::*;
use poem::{EndpointExt, Route, Server, post};

use ottel_spaniel::{Format, Sink, Stats};

mod collect;
mod search;

pub struct Options {
    pub host: &'static str,
    pub port: u16,
    pub shutdown_timeout_secs: u8,
}

impl Options {
    fn addr(&self) -> impl tokio::net::ToSocketAddrs {
        (self.host, self.port)
    }

    fn shutdown_timeout(&self) -> Duration {
        Duration::from_secs(self.shutdown_timeout_secs.into())
    }
}

pub async fn run_server(options: Options, format: Format, stats: Stats, sink: Sink) {
    use collect::*;
    use search::*;

    let routes = Route::new()
        .at("/v1/traces", post(v1_handle_export_trace_request))
        .at("/v0/search/span", post(v0_search_traces))
        .at("/v0/search/span/name", post(v0_search_get_span_names))
        .at("/v0/search/resource/name", post(v0_search_get_svc_names))
        .with(Cors::default())
        .with(AddData::new(sink))
        .with(AddData::new(stats))
        .with(AddData::new(format));

    let tcp = TcpListener::bind(options.addr());
    let server = Server::new(tcp);
    let signal = async {
        tokio::signal::ctrl_c().await.unwrap();
    };

    server
        .run_with_graceful_shutdown(routes, signal, Some(options.shutdown_timeout()))
        .await
        .expect("server.closes");
}
