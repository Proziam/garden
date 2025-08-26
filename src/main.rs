use anyhow::Result;
use axum::{
    Router,
    extract::{Path as AxumPath, State},
    response::Html,
    routing::get,
};
use chrono::NaiveDate;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use pulldown_cmark::{Options, Parser, html};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tera::{Context, Tera};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber;

struct AppState {
    tera: Tera,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BlogPost {
    title: String,
    date: String,
    subtitle: Option<String>,
    tags: Vec<String>,
    excerpt: String,
    reading_time: u32,
    slug: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: String,
    date: NaiveDate,
    subtitle: Option<String>,
    tags: Vec<String>,
    excerpt: String,
    reading_time: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut tera = Tera::new("templates/**/*.html")?;
    tera.autoescape_on(vec![".html"]);

    let shared_state = Arc::new(AppState { tera });

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/posts", get(blog_index_handler))
        .route("/posts/{slug}", get(blog_post_handler))
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

async fn blog_index_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let posts = load_blog_posts().unwrap_or_else(|e| {
        tracing::error!("Failed to load blog posts: {}", e);
        Vec::new()
    });

    let mut context = Context::new();
    context.insert("posts", &posts);

    let html = state.tera.render("blog.html", &context).unwrap();
    Html(html)
}

async fn blog_post_handler(
    State(state): State<Arc<AppState>>,
    AxumPath(slug): AxumPath<String>,
) -> Html<String> {
    let post_path = Path::new("posts").join(format!("{}.md", slug));

    match load_single_post(&post_path) {
        Ok(post) => {
            let mut context = Context::new();
            context.insert("post", &post);

            let html = state
                .tera
                .render("post.html", &context)
                .unwrap_or_else(|e| {
                    tracing::error!("Failed to render post template: {}", e);
                    format!("<h1>Error rendering post</h1><p>{}</p>", e)
                });
            Html(html)
        }
        Err(e) => {
            tracing::error!("Failed to load post {}: {}", slug, e);
            Html(format!(
                "<h1>Post not found</h1><p>The post '{}' could not be found.</p>",
                slug
            ))
        }
    }
}

fn load_blog_posts() -> Result<Vec<BlogPost>> {
    let posts_dir = Path::new("posts");
    let mut posts = Vec::new();

    if posts_dir.exists() {
        for entry in fs::read_dir(posts_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(post) = load_single_post(&path) {
                    posts.push(post);
                }
            }
        }
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(posts)
}

fn load_single_post(path: &Path) -> Result<BlogPost> {
    let content = fs::read_to_string(path)?;
    let matter = Matter::<YAML>::new();
    let parsed = matter.parse(&content);

    let frontmatter: FrontMatter = parsed
        .data
        .ok_or_else(|| anyhow::anyhow!("No frontmatter found"))?
        .deserialize()?;

    let slug = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(&parsed.content, options);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    Ok(BlogPost {
        title: frontmatter.title,
        date: frontmatter.date.format("%B %d, %Y").to_string(),
        subtitle: frontmatter.subtitle,
        tags: frontmatter.tags,
        excerpt: frontmatter.excerpt,
        reading_time: frontmatter.reading_time,
        slug,
        content: html_content,
    })
}
