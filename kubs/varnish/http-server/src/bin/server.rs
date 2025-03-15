use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::http::{ContentType, Status};
use rocket::response::content::RawJson;
use rocket::response::status::Custom;
use rocket::{get, routes, Data, Request, Response, State};

use http_server::{db, otel};

enum Load {
    Db(db::Message),
    Exit,
}

struct Hive {
    vec: Vec<std::sync::mpsc::Sender<Load>>,
    han: Vec<std::thread::JoinHandle<()>>,
}

impl Hive {
    fn end(self) {
        for c in self.vec {
            c.send(Load::Exit).unwrap();
        }
        for h in self.han {
            h.join().unwrap();
        }
    }
}

struct Bees {
    idx: std::sync::atomic::AtomicUsize,
    siz: usize,
    vec: Vec<std::sync::mpsc::Sender<Load>>,
}

impl Bees {
    fn new(size: usize) -> (Self, Hive) {
        let mut vec = Vec::with_capacity(size);
        let mut han = Vec::with_capacity(size);

        for _i in 0..size {
            let (tx, rx) = std::sync::mpsc::channel();
            vec.push(tx);
            let thread = std::thread::spawn(move || {
                let conn = db::connect(&db::path());

                loop {
                    let msg = match rx.recv() {
                        Ok(Load::Db(m)) => m,
                        Ok(Load::Exit) => return,
                        Err(e) => {
                            tracing::error!("Channel error {}", e);
                            return;
                        }
                    };
                    db::exec(&conn, msg);
                }
            });
            han.push(thread);
        }

        (
            Self {
                idx: AtomicUsize::new(0),
                siz: size,
                vec: vec.clone(),
            },
            Hive { han, vec },
        )
    }

    fn send(&self, msg: db::Message) {
        let idx = self.idx.fetch_add(1, Ordering::Relaxed);
        self.vec[idx % self.siz].send(Load::Db(msg)).unwrap();
    }
}

const FAVICON: &[u8] = include_bytes!("./favicon.png");

async fn get_json_obj(res: db::Response) -> Result<RawJson<String>, Custom<String>> {
    let result = match res.result.await {
        Ok(r) => r,
        Err(error) => {
            tracing::error!("DB Channel bug: {}", error);
            return Err(Custom(
                Status::InternalServerError,
                String::from("THIS IS A BUG"),
            ));
        }
    };

    let mut rows = match result {
        Ok(r) => r,
        Err(error) => {
            tracing::error!("DB Query bug: {}", error);
            return Err(Custom(
                Status::InternalServerError,
                String::from("THIS IS A QUERY BUG"),
            ));
        }
    };

    if rows.is_empty() {
        tracing::debug!("Item not found in DB.");
        return Err(Custom(Status::NotFound, String::from("Not found")));
    }

    match rows.pop() {
        Some(db::TheRow::Json { value }) => Ok(RawJson(value)),
        None => {
            tracing::error!("DB Row kind bug, expected JSON got NONE");
            Err(Custom(
                Status::InternalServerError,
                String::from("THIS IS A ROW KIND BUG"),
            ))
        }
    }
}

#[get("/favicon.ico")]
#[tracing::instrument(parent = &ctx.span)]
fn favicon(ctx: &RequestCtx) -> (Status, (ContentType, &'static [u8])) {
    (Status::Ok, (ContentType::PNG, FAVICON))
}

#[get("/static/fast")]
#[tracing::instrument(parent = &ctx.span)]
fn fast_static(ctx: &RequestCtx) -> &'static str {
    tracing::info!("Fastness");
    "Sonic"
}

#[get("/static/slow")]
#[tracing::instrument]
async fn slow_static() -> &'static str {
    tracing::info!("Slowness");
    rocket::tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    "I am so slow"
}

#[get("/pokedex/<lang>/ability/<id>")]
#[tracing::instrument(parent = &ctx.span, skip(ctx, sender))]
async fn get_ability(
    ctx: &RequestCtx,
    lang: &str,
    id: u16,
    sender: &State<Bees>,
) -> Result<RawJson<String>, Custom<String>> {
    let (res, msg) = db::queries::get_ability(lang, id);
    sender.inner().send(msg);
    get_json_obj(res).await
}

#[get("/pokedex/<lang>/type/<id>")]
#[tracing::instrument(parent = &ctx.span, skip(ctx, sender))]
async fn get_type(
    ctx: &RequestCtx,
    lang: &str,
    id: u16,
    sender: &State<Bees>,
) -> Result<RawJson<String>, Custom<String>> {
    let (res, msg) = db::queries::get_type(lang, id);
    sender.inner().send(msg);
    get_json_obj(res).await
}

#[get("/pokedex/<lang>/item/<id>")]
#[tracing::instrument(parent = &ctx.span, skip(ctx, sender))]
async fn get_item(
    ctx: &RequestCtx,
    lang: &str,
    id: u16,
    sender: &State<Bees>,
) -> Result<RawJson<String>, Custom<String>> {
    let (res, msg) = db::queries::get_item(lang, id);
    sender.inner().send(msg);
    get_json_obj(res).await
}

#[get("/pokedex/<lang>/move/<id>")]
#[tracing::instrument(parent = &ctx.span, skip(ctx, sender))]
async fn get_move(
    ctx: &RequestCtx,
    lang: &str,
    id: u16,
    sender: &State<Bees>,
) -> Result<RawJson<String>, Custom<String>> {
    let (res, msg) = db::queries::get_move(lang, id);
    sender.inner().send(msg);
    get_json_obj(res).await
}

#[get("/pokedex/<lang>/pokemon/<id>")]
#[tracing::instrument(parent = &ctx.span, skip(ctx, sender))]
async fn get_pokemon(
    ctx: &RequestCtx,
    lang: &str,
    id: u16,
    sender: &State<Bees>,
) -> Result<RawJson<String>, Custom<String>> {
    let (res, msg) = db::queries::get_pokemon(lang, id);
    sender.inner().send(msg);
    get_json_obj(res).await
}

#[derive(Debug)]
struct RequestCtx {
    start: std::time::Instant,
    span: tracing::Span,
}

impl RequestCtx {
    fn new(span: tracing::Span) -> Self {
        Self {
            span,
            start: std::time::Instant::now(),
        }
    }
}

#[rocket::async_trait]
impl<'a> rocket::request::FromRequest<'a> for &'a RequestCtx {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        rocket::request::Outcome::Success(
            request.local_cache(|| unreachable!("RequestCtx must be available")),
        )
    }
}

struct Metered {
    req_time: opentelemetry::metrics::Histogram<f64>,
}

impl Metered {
    fn new() -> Self {
        Self {
            req_time: opentelemetry::global::meter_provider()
                .meter(env!("CARGO_PKG_NAME"))
                .f64_histogram("http.server.request.duration")
                .with_description("Request time histogram")
                .with_unit("second")
                .build(),
        }
    }
}

#[rocket::async_trait]
impl rocket::fairing::Fairing for Metered {
    fn info(&self) -> rocket::fairing::Info {
        use rocket::fairing::{Info, Kind};

        Info {
            name: "OTEL metrics",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        use tracing::field::Empty;

        let span = tracing::info_span!(
            "http.server.request",
            http.request.method = Empty,
            server.address = Empty,
            server.port = Empty,
            http.route = Empty,
            http.response.status_code = Empty,
        );
        // Insert RequestCtx into request local_cache.
        let _ctx = request.local_cache(|| RequestCtx::new(span));
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        use opentelemetry::KeyValue;

        let ctx: &RequestCtx = request.local_cache(|| unreachable!("RequestCtx must be available"));
        let _g = ctx.span.enter();

        let attr_request_method = request.method().as_str();
        let attr_status_code = response.status().code.to_string();
        let attr_route = request
            .route()
            .map(|r| r.uri.to_string())
            .unwrap_or(String::from("unknown"));

        let mut attrs = vec![
            KeyValue::new("http.request.method", attr_request_method),
            KeyValue::new("http.response.status_code", attr_status_code.clone()),
            KeyValue::new("http.route", attr_route.clone()),
        ];

        ctx.span.record("http.request.method", attr_request_method);
        ctx.span
            .record("http.response.status_code", attr_status_code);
        ctx.span.record("http.route", attr_route);

        if let Some(host) = request.host() {
            let attr_domain = host.domain().as_str().to_owned();
            let attr_port = host
                .port()
                .map(|v| v.to_string())
                .unwrap_or(String::from("unknown"));

            attrs.extend_from_slice(&[
                KeyValue::new("server.address", attr_domain.clone()),
                KeyValue::new("server.port", attr_port.clone()),
            ]);
            ctx.span.record("server.address", attr_domain);
            ctx.span.record("server.port", attr_port);
        } else {
            attrs.extend_from_slice(&[
                KeyValue::new("server.address", "unknown"),
                KeyValue::new("server.port", "unknown"),
            ]);
        }

        self.req_time
            .record(ctx.start.elapsed().as_secs_f64(), &attrs);
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _logger_provider = otel::init();

    let (beez, hive) = Bees::new(4);
    let http_metrics = Metered::new();

    let rock = rocket::build()
        .attach(http_metrics)
        .manage(beez)
        .mount(
            "/",
            routes![
                favicon,
                fast_static,
                slow_static,
                get_ability,
                get_type,
                get_item,
                get_move,
                get_pokemon,
            ],
        )
        .launch();

    rock.await?;
    hive.end();

    Ok(())
}
