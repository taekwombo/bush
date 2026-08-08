//! 1. Asks user for input.
//! 2. Prepares propagation context.
//! 3. Spawns this example once more with propagation context provided.
//! 4. Child reads context, extracts baggage, prints baggage value.

use std::collections::HashMap;

use opentelemetry::baggage::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_opentelemetry::OpenTelemetrySpanExt;

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .with(trayray::otel_trace_layer())
        .with(tracing_subscriber::fmt::layer())
        .init();

    trayray::otel_init_propagation();

    match read_config() {
        Config::Master { exe_path } => run_master(exe_path),
        Config::Slave { echoing } => run_slave(echoing),
    }
}

enum Config {
    Master {
        exe_path: String,
    },
    Slave {
        echoing: String,
    }
}

fn read_config() -> Config {
    let mut args = std::env::args();
    tracing::info!(args = args.len(), "read_config");

    if args.len() == 1 {
        return Config::Master {
            exe_path: args.next().unwrap(),
        };
    }

    if args.len() == 2 {
        return Config::Slave {
            echoing: args.last().unwrap(),
        };
    }

    unreachable!("One or two arguments supported");
}

fn run_master(exe_path: String) {
    use std::io::Write;

    let span = tracing::info_span!("master-baggage");
    let _activ = span.enter();

    let mut input = String::new();
    eprintln!(">> Say something:");
    std::io::stdin().read_line(&mut input).unwrap();

    let mut baggage = Baggage::new();
    baggage.insert("user-input", input);

    let baggaged = opentelemetry::Context::current_with_baggage(baggage);
    let _active = baggaged.attach();
    
    let mut carrier: HashMap<String, String> = HashMap::new();
    opentelemetry::global::get_text_map_propagator(|propagator| propagator.inject(&mut carrier));

    let output = std::process::Command::new(exe_path)
        .args([serialize(&carrier)])
        .output()
        .unwrap();

    std::io::stdout().write_all(output.stdout.as_ref()).unwrap();
    std::io::stderr().write_all(output.stderr.as_ref()).unwrap();
}

fn run_slave(encoded_context: String) {
    let span = tracing::info_span!("slave-baggage");

    let carrier = deserialize(&encoded_context);    
    let context = opentelemetry::global::get_text_map_propagator(|propagator| propagator.extract(&carrier));

    span.set_parent(context).unwrap();

    let _activ = span.enter();
    let ctx = opentelemetry::Context::current();
    let user_input = ctx.baggage().get("user-input");
    let user_input = user_input.map(|a| a.as_str()).unwrap_or("");

    tracing::info!("User said to say: {}", user_input);
}

fn serialize(carrier: &HashMap<String, String>) -> String {
    use base64::Engine;

    let engine = base64::engine::general_purpose::STANDARD;

    tracing::debug!(?carrier, "Serializing");

    let mut output = String::new();

    for (key, value) in carrier {
        let line = format!("{}:{}", engine.encode(key), engine.encode(value));

        if !output.is_empty() {
            output.push(';');
        }

        output.push_str(&line);
    }

    output
}

fn deserialize(encoded: &str) -> HashMap<String, String> {
    use base64::Engine;

    let engine = base64::engine::general_purpose::STANDARD;

    let mut carrier: HashMap<String, String> = HashMap::new();

    for part in encoded.split(';') {
        let mut kv = part.split(':');
        let key = engine.decode(kv.next().unwrap()).unwrap();
        let val = engine.decode(kv.next().unwrap()).unwrap();

        carrier.insert(String::from_utf8(key).unwrap(), String::from_utf8(val).unwrap());
    }

    tracing::debug!(?carrier, "Deserialized");

    carrier
}
