//! Running:
//! ```sql
//! CREATE TABLE theperson(id BIGINT PRIMARY KEY, name TEXT);
//! CREATE PUBLICATION user_pub FOR TABLE ONLY theperson;
//! ```

use std::error::Error;

use pg_replicate::pipeline::batching::data_pipeline::BatchDataPipeline;
use pg_replicate::pipeline::batching::BatchConfig;
use pg_replicate::pipeline::sinks::*;
use pg_replicate::pipeline::sources::postgres::*;
use pg_replicate::pipeline::*;
use pg_replicate::table::{TableId, TableSchema};
use pg_replicate::conversions::cdc_event::CdcEvent;
use pg_replicate::conversions::table_row::TableRow;
use postgres_types::PgLsn;

fn init_tracing() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
}

struct GigaSink;

#[async_trait::async_trait]
impl BatchSink for GigaSink {
    type Error = InfallibleSinkError;

    async fn get_resumption_state(&mut self) -> Result<PipelineResumptionState, Self::Error> {
        // Resuming from the very beginning.
        Ok(PipelineResumptionState {
            copied_tables: std::collections::HashSet::new(),
            last_lsn: PgLsn::from(0),
        })
    }

    async fn write_table_schemas(&mut self, table_schemas: std::collections::HashMap<TableId, TableSchema>) -> Result<(), Self::Error> {
        tracing::info!("Table schemas: {table_schemas:#?}");
        Ok(())
    }

    async fn write_table_rows(&mut self, rows: Vec<TableRow>, table_id: TableId) -> Result<(), Self::Error> {
        tracing::info!("Table rows: {table_id:?} {rows:#?}");
        Ok(())
    }

    async fn write_cdc_events(&mut self, events: Vec<CdcEvent>) -> Result<tokio_postgres::types::PgLsn, Self::Error> {
        tracing::info!("CDC events: {events:#?}");
        Ok(PgLsn::from(0))
    }

    async fn table_copied(&mut self, table_id: TableId) -> Result<(), Self::Error> {
        tracing::info!("Table copied: {table_id:?}");
        Ok(())
    }

    async fn truncate_table(&mut self, table_id: TableId) -> Result<(), Self::Error> {
        tracing::info!("Table truncated: {table_id:?}");
        Ok(())
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_tracing();

    let host = "localhost";
    let port = 45432;
    let database = "db";
    let username = "user";
    let password = Some("pass".to_string());
    let slot_name = Some("replinho".to_string());
    let table_names = TableNamesFrom::Publication("user_pub".to_string());

    let postgres_source = PostgresSource::new(
        host,
        port,
        database,
        username,
        password,
        slot_name,
        table_names,
    )
    .await?;

    let mut pipeline = BatchDataPipeline::new(postgres_source, GigaSink, PipelineAction::Both, BatchConfig::new(4, std::time::Duration::from_secs(5)));

    pipeline.start().await?;

    Ok(())
}
