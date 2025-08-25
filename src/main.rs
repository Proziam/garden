use anyhow::Result;
use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};
use std::sync::Arc;
use tera::{Context, Tera};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber;

struct AppState {
    tera: Tera,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut tera = Tera::new("templates/**/*.html")?;
    tera.autoescape_on(vec![".html"]);
    
    let shared_state = Arc::new(AppState { tera });

    let app = Router::new()
        .route("/", get(home_handler))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("Server running at http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn home_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let context = Context::new();
    let html = state.tera.render("home.html", &context).unwrap();
    Html(html)
}