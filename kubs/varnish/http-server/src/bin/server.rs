use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{get, routes, State};
use rocket::response::content::RawJson;
use rocket::response::status::Custom;
use rocket::http::{ContentType, Status};

use http_server::db;

enum Load {
    Db(db::Message),
    Exit,
}

struct Hive {
    vec: Vec<std::sync::mpsc::Sender<Load>>,
    han: Vec<std::thread::JoinHandle<()>>,
}

impl Hive {
    fn end(self) -> () {
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

        for i in 0..size {
            let (tx, rx) = std::sync::mpsc::channel();
            vec.push(tx);
            let thread = std::thread::spawn(move || {
                let span = tracing::info_span!("worker::db", index = i);
                let _g = span.enter();
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
                    tracing::info!("Message");
                }
            });
            han.push(thread);
        }

        (Self { idx: AtomicUsize::new(0), siz: size, vec: vec.clone() }, Hive { han, vec })
    }

    fn send(&self, msg: db::Message) {
        let idx = self.idx.fetch_add(1, Ordering::Relaxed);
        self.vec[idx % self.siz].send(Load::Db(msg)).unwrap();
    }
}

const FAVICON: &'static [u8] = include_bytes!("./favicon.png");

async fn get_json_obj(res: db::Response) -> Result<RawJson<String>, Custom<String>> {
    let result = match res.result.await {
        Ok(r) => r,
        Err(error) => {
            tracing::error!("DB Channel bug: {}", error);
            return Err(Custom(Status::InternalServerError, String::from("THIS IS A BUG")));
        }
    };

    let mut rows = match result {
        Ok(r) => r,
        Err(error) => {
            tracing::error!("DB Query bug: {}", error);
            return Err(Custom(Status::InternalServerError, String::from("THIS IS A QUERY BUG")));
        }
    };

    if rows.is_empty() {
        return Err(Custom(Status::NotFound, String::from("Not found")));
    }

    match rows.pop() {
        Some(db::TheRow::Json { value }) => Ok(RawJson(value)),
        None => {
            tracing::error!("DB Row kind bug, expected JSON got NONE");
            Err(Custom(Status::InternalServerError, String::from("THIS IS A ROW KIND BUG")))
        }
    }
}

#[get("/favicon.ico")]
fn favicon() -> (Status, (ContentType, &'static [u8])) {
    (Status::Ok, (ContentType::PNG, FAVICON))
}

#[get("/static/fast")]
fn fast_static() -> &'static str {
    tracing::info!("Fastness");
    "Sonic"
}

#[get("/static/slow")]
async fn slow_static() -> &'static str {
    tracing::info!("Slowness");
    rocket::tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    "I am so slow"
}

#[get("/pokedex/<lang>/ability/<id>")]
async fn get_ability(lang: &str, id: u16, sender: &State<Bees>) -> Result<RawJson<String>, Custom<String>> {
    let (res, msg) = db::queries::get_ability(lang, id);
    sender.inner().send(msg);
    get_json_obj(res).await
}

#[get("/pokedex/<lang>/type/<id>")]
async fn get_type(lang: &str, id: u16, sender: &State<Bees>) -> Result<RawJson<String>, Custom<String>> {
    let (res, msg) = db::queries::get_type(lang, id);
    sender.inner().send(msg);
    get_json_obj(res).await
}

#[get("/pokedex/<lang>/item/<id>")]
async fn get_item(lang: &str, id: u16, sender: &State<Bees>) -> Result<RawJson<String>, Custom<String>> {
    let (res, msg) = db::queries::get_item(lang, id);
    sender.inner().send(msg);
    get_json_obj(res).await
}

#[get("/pokedex/<lang>/move/<id>")]
async fn get_move(lang: &str, id: u16, sender: &State<Bees>) -> Result<RawJson<String>, Custom<String>> {
    let (res, msg) = db::queries::get_move(lang, id);
    sender.inner().send(msg);
    get_json_obj(res).await
}

#[get("/pokedex/<lang>/pokemon/<id>")]
async fn get_pokemon(lang: &str, id: u16, sender: &State<Bees>) -> Result<RawJson<String>, Custom<String>> {
    let (res, msg) = db::queries::get_pokemon(lang, id);
    sender.inner().send(msg);
    get_json_obj(res).await
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    tracing_subscriber::fmt::init();

    let (beez, hive) = Bees::new(4);

    let rock = rocket::build()
        .manage(beez)
        .mount("/", routes![
            favicon,
            fast_static,
            slow_static,
            get_ability,
            get_type,
            get_item,
            get_move,
            get_pokemon,
        ])
        .launch();

    rock.await?;
    hive.end();

    Ok(())
}
