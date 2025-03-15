use duckdb::Connection;

#[cfg(debug_assertions)]
pub fn path() -> String {
    std::env::var("DB_PATH").unwrap_or("./pokeapi.db".to_owned())
}

#[cfg(not(debug_assertions))]
pub fn path() -> String {
    std::env::var("DB_PATH").unwrap()
}

#[tracing::instrument(skip(conn))]
fn create_macros(conn: &Connection) {
    macro_rules! inc {
        ($c:ident, $id:expr) => {
            tracing::info_span!("db.execute", file_name = $id)
                .in_scope(|| $c.execute(include_str!($id), []).unwrap());
        };
    }

    inc!(conn, "./db/00.get_version.sql");
    inc!(conn, "./db/01.get_generation.sql");
    inc!(conn, "./db/02.get_type_base.sql");
    inc!(conn, "./db/03.get_type.sql");
    inc!(conn, "./db/04.get_ability.sql");
    inc!(conn, "./db/05.get_item_pocket.sql");
    inc!(conn, "./db/06.get_item_fling.sql");
    inc!(conn, "./db/07.get_item_category.sql");
    inc!(conn, "./db/08.get_item.sql");
    inc!(conn, "./db/09.get_move_target.sql");
    inc!(conn, "./db/10.get_move_damage_class.sql");
    inc!(conn, "./db/11.get_move_effect.sql");
    inc!(conn, "./db/12.get_contest_type.sql");
    inc!(conn, "./db/13.get_contest_effect.sql");
    inc!(conn, "./db/14.get_super_contest_effect.sql");
    inc!(conn, "./db/15.get_move_meta.sql");
    inc!(conn, "./db/16.get_move.sql");
    inc!(conn, "./db/17.get_egg_group.sql");
    inc!(conn, "./db/18.get_pokemon_move.sql");
    inc!(conn, "./db/19.get_pokemon_move_meta.sql");
    inc!(conn, "./db/20.get_pokemon.sql");
}

#[tracing::instrument]
pub fn connect(path: &str) -> Connection {
    use duckdb::{AccessMode, Config};

    let cfg = Config::default()
        .access_mode(AccessMode::ReadOnly)
        .expect("DuckDB config");
    let conn = Connection::open_with_flags(path, cfg).expect("DuckDB connection");

    create_macros(&conn);

    conn
}

pub struct Message {
    parent: tracing::Span,
    query: &'static str,
    params: Vec<Box<dyn duckdb::ToSql + Send>>,
    tx: rocket::tokio::sync::oneshot::Sender<Result<Vec<TheRow>, String>>,
    map_row: fn(&duckdb::Row<'_>) -> duckdb::Result<TheRow>,
}

#[derive(Debug)]
pub enum TheRow {
    Json { value: String },
}

pub struct Response {
    pub result: rocket::tokio::sync::oneshot::Receiver<Result<Vec<TheRow>, String>>,
}

pub mod queries {
    use super::*;
    use rocket::tokio::sync::oneshot::channel;

    fn single_column_json_str(row: &duckdb::Row<'_>) -> duckdb::Result<TheRow> {
        Ok(TheRow::Json { value: row.get(0)? })
    }

    fn get_single(lang: &str, id: u16, query: &'static str) -> (Response, Message) {
        let (tx, rx) = channel();
        let message = Message {
            parent: tracing::Span::current(),
            tx,
            query,
            params: vec![Box::new(lang.to_owned()), Box::new(id)],
            map_row: single_column_json_str,
        };

        (Response { result: rx }, message)
    }

    #[tracing::instrument]
    pub fn get_ability(lang: &str, id: u16) -> (Response, Message) {
        const QUERY: &str = "SELECT data::JSON FROM get_ability(?, ?)";
        get_single(lang, id, QUERY)
    }

    #[tracing::instrument]
    pub fn get_type(lang: &str, id: u16) -> (Response, Message) {
        const QUERY: &str = "SELECT data::JSON FROM get_type(?, ?)";
        get_single(lang, id, QUERY)
    }

    #[tracing::instrument]
    pub fn get_item(lang: &str, id: u16) -> (Response, Message) {
        const QUERY: &str = "SELECT data::JSON FROM get_item(?, ?)";
        get_single(lang, id, QUERY)
    }

    #[tracing::instrument]
    pub fn get_move(lang: &str, id: u16) -> (Response, Message) {
        const QUERY: &str = "SELECT data::JSON FROM get_move(?, ?)";
        get_single(lang, id, QUERY)
    }

    #[tracing::instrument]
    pub fn get_pokemon(lang: &str, id: u16) -> (Response, Message) {
        const QUERY: &str = "SELECT data::JSON FROM get_pokemon(?, ?)";
        get_single(lang, id, QUERY)
    }
}

static QUERY_TIME_HISTOGRAM: std::sync::OnceLock<opentelemetry::metrics::Histogram<f64>> =
    std::sync::OnceLock::new();

fn get_hist() -> &'static opentelemetry::metrics::Histogram<f64> {
    QUERY_TIME_HISTOGRAM.get_or_init(|| {
        opentelemetry::global::meter_provider()
            .meter(env!("CARGO_PKG_NAME"))
            .f64_histogram("duckdb.query.duration")
            .with_description("Query time histogram")
            .with_unit("second")
            .build()
    })
}

pub fn exec(conn: &Connection, message: Message) {
    let span = tracing::info_span!(parent: &message.parent, "exec");
    let _g = span.enter();
    let query_start = std::time::Instant::now();

    let mut statement = match conn.prepare_cached(message.query) {
        Ok(r) => r,
        Err(error) => {
            tracing::error!(error = %error, ".prepare_cached");
            message
                .tx
                .send(Err(format!(
                    "Failed to prepare cached statement: {}.",
                    error
                )))
                .expect("Send");
            return;
        }
    };
    let params = duckdb::params_from_iter(message.params);
    let query_result = match statement.query(params) {
        Ok(r) => r,
        Err(error) => {
            tracing::error!(error = %error, ".query");
            message
                .tx
                .send(Err(format!("Failed to execute statement: {}.", error)))
                .expect("Send");
            return;
        }
    };
    let mut results = Vec::new();
    for row in query_result.mapped(message.map_row) {
        match row {
            Ok(r) => results.push(r),
            Err(error) => {
                tracing::error!(error = %error, ".map_row");
                message.tx.send(Err(error.to_string())).expect("Send");
                return;
            }
        }
    }
    get_hist().record(
        query_start.elapsed().as_secs_f64(),
        &[opentelemetry::KeyValue::new("query_string", message.query)],
    );
    message.tx.send(Ok(results)).expect("Send");
}
