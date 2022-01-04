#[macro_use]
extern crate rocket;

use ::rocket::State;
use mongodb::{bson::doc, Client, Database};

// pool struct
struct PoolDb {
    db: Database,
}

#[get("/")]
fn index() -> &'static str {
    "Hello from Rust"
}

// a call to api will test if you connect succesfully to database
#[get("/api")]
async fn api(pool: &State<PoolDb>) -> &'static str {
    match pool.db.run_command(doc! {"ping": 1}, None).await {
        Ok(_) => "successsss",
        Err(_) => "F something gone wrong",
    }
}

fn env(inp: &str) -> String {
    match std::env::var(inp) {
        Ok(value) => value,
        Err(_) => panic!("key {} not found", inp),
    }
}
#[launch]
async fn rocket() -> _ {
    let uri = env("MONGO_URI");
    let db_str = env("MONGO_DATABASE");
    // mongo connection
    let client = Client::with_uri_str(&uri[..]).await.ok();
    let db = client.unwrap().database(&db_str[..]);
    //let collection = db.collection::<Document>("url");

    // implement a pool to mongo db connection using manage
    rocket::build()
        .mount("/", routes![api, index])
        .manage(PoolDb { db: db })
}
