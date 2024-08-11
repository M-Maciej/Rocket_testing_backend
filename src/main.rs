#[macro_use] extern crate rocket;

mod rocket_webserver;
mod routes;

use rocket_webserver::*;

#[tokio::main] // or #[rocket::main] depending on your runtime setup
async fn main() {
    match launch_rocket().await.launch().await {
        Ok(_) => println!("Server closed gracefully"),
        Err(error) => println!("{:?}", error),
    }
}
