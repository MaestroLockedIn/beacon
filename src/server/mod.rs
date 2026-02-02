use tokio::net::TcpListener;

pub mod handler;
pub mod models;
pub mod routes;
pub mod service;

pub async fn start() {
    let url = String::from("0.0.0.0:5678");
    let listener = TcpListener::bind(&url).await.unwrap();
    println!("Server started at : {}", &url);
    let routes = routes::configure_routes();
    axum::serve(listener, routes).await.unwrap()
}
