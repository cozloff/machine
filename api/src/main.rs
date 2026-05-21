mod handlers;
mod infrastructure;
mod models;
mod repositories;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let app = routes::create_router();
    let bind_address =
        std::env::var("API_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
