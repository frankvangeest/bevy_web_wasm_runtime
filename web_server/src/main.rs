use axum::Router;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use hyper::http::{HeaderName, HeaderValue};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Serve files from ./dist (your WASM output directory)
    let static_files = ServeDir::new("public")
        .not_found_service(ServeDir::new("public/index.html"));

    // Add the WebGPU-required headers
    let app = Router::new()
        .fallback_service(static_files)
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("cross-origin-opener-policy"),
            HeaderValue::from_static("same-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("cross-origin-embedder-policy"),
            HeaderValue::from_static("require-corp"),
        ));

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("🚀 WebGPU server running at http://{addr}");

    // Hyper 1.0 way:
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
