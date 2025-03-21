use dotenv::dotenv;
use std::sync::Arc;

use thunder_rs::{
    http::{
        httpmethod::HttpMethod,
        routes::{ContentHeader, MiddlewareArc},
    },
    server::server::Server,
};

mod handlers;
mod structs;

use handlers::{handle_callback, handle_payment};

#[tokio::main]
async fn main() {
    let mut server = Server::new("127.0.0.1:8000");

    println!("Server starting on port 8000");

    dotenv().ok();

    let http_server = server.http_server();

    http_server.router(
        HttpMethod::POST,
        "/payment",
        handle_payment,
        None::<MiddlewareArc>,
        Some(ContentHeader::ApplicationJson),
    );

    http_server.router(
        HttpMethod::POST,
        "/paymentCallback",
        handle_callback,
        None::<MiddlewareArc>,
        Some(ContentHeader::ApplicationJson),
    );

    let server = Arc::new(server);

    server.start().await.unwrap();
}
