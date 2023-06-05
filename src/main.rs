mod user;

use std::{env, sync::Arc};

use crate::user::user_routes;
use actix_web::{get, web::Data, App, HttpServer, Responder, Result};
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[get("/")]
async fn index() -> impl Responder {
    "Hello world"
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let url_database = env::var("URL_DATABASE").expect("URL_DATABASE must be set");

    let (client, connection) = match tokio_postgres::connect(&url_database, NoTls).await {
        Ok(connection) => connection,
        Err(error) => panic!("Error Connect Database: {}", error),
    };

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let client = Arc::new(client);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .service(index)
            .configure(user_routes::routes)
    })
    .bind(format!("{host}:{port}"))?
    .run()
    .await
}
