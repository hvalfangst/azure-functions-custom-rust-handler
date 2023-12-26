mod arithmetic;

use std::env;
use crate::arithmetic::router::router::arithmetic_routes;

#[tokio::main]
async fn main() {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";

    // Read the port from the environment variable or use a default value
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let bind_address = format!("0.0.0.0:{}", port);

    axum::Server::bind(&bind_address.parse().unwrap())
        .serve(arithmetic_routes().into_make_service())
        .await
        .unwrap();
}