mod flag {
    pub const SERVICE_NAME: &str = "service-name";
    pub const FORWARD_TO: &str = "forward-to";
}

// When there is an option given:
// - forward - each request will be forwarde to the address
// - service.name - 
// - OTEL address is hardcoded to http://localhost
fn main() {
    println!("{:?}", std::env::args());
    println!("{:?}", load_config());

    use opentelemetry_otlp::SpanExporter;

    eprintln!("{:#?}", SpanExporter::builder().build().unwrap());
}

#[derive(Debug)]
enum Config {
    Master {
        bin: std::path::PathBuf,
    },
    Slave {
        forward_to: Option<std::net::SocketAddrV4>,
        service_name: String,
    }
}

fn load_config() -> Config {
    use std::net::SocketAddrV4;
    use std::path::PathBuf;
    use std::str::FromStr;

    let mut args = std::env::args();

    if args.len() == 1 {
        return Config::Master {
            bin: PathBuf::from(args.next().unwrap()),
        };
    }

    let service_name = args
        .find(|a| a.starts_with(flag::SERVICE_NAME))
        .map(|a| {
            let (_, val) = a.split_at(flag::SERVICE_NAME.len() + 1);
            val.to_string()
        })
        .expect("missing service name");
    let forward_to = args
        .find(|a| a.starts_with(flag::FORWARD_TO))
        .map(|a| {
            let (_, val) = a.split_at(flag::FORWARD_TO.len() + 1);
            SocketAddrV4::from_str(val)
        })
        .map(Result::unwrap);

    Config::Slave {
        forward_to,
        service_name,
    }
}
