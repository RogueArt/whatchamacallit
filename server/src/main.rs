#[macro_use] extern crate rocket;

// Set up connection to Mongo
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello from map diff!"
}

async fn mongo_connect() -> mongodb::error::Result<()> {
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb+srv://<username>:<password>@<cluster-url>/test?w=majority")
            .await?;
    // Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");
    // List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}