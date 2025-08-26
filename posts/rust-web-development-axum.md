---
title: Rust and Web Development with Axum
date: 2024-08-25
subtitle: My journey building web applications with Rust and the Axum framework
tags:
  - rust
  - web-development
  - axum
  - programming
excerpt: After years of building web apps in JavaScript and Python, I decided to explore Rust for web development. Here's what I learned building my first production app with Axum.
reading_time: 7
---

# Rust and Web Development with Axum

Coming from a background in Node.js and Python web frameworks, Rust seemed like an unlikely choice for web development. But after building my first production application with Axum, I'm convinced it's a fantastic option for certain use cases.

## Why Rust for Web Development?

The typical concerns about Rust for web development are:
- Steep learning curve
- Longer development time
- Overkill for CRUD apps

While these are valid concerns, Rust offers unique advantages:
- **Performance**: Blazing fast response times
- **Memory safety**: No null pointer exceptions in production
- **Concurrency**: Fearless concurrent programming
- **Type safety**: Catch errors at compile time

## Enter Axum

Axum is a web framework built on top of Tower and Hyper, focusing on:
- Ergonomic API design
- Type-safe routing
- Excellent middleware support
- Great performance characteristics

## Building My First App

Here's what I built: a digital garden/blog platform (yes, this very site!).

### Project Structure

```rust
src/
├── main.rs           // Application entry point
├── routes/           // HTTP route handlers
├── models/           // Data structures
├── templates/        // HTML templates
└── utils/            // Helper functions
```

### Key Learnings

#### 1. The Type System is Your Friend

Initially frustrating, Rust's type system becomes invaluable:

```rust
#[derive(Deserialize)]
struct CreatePost {
    title: String,
    content: String,
    tags: Vec<String>,
}

async fn create_post(
    Json(payload): Json<CreatePost>,
) -> Result<impl IntoResponse, AppError> {
    // Guaranteed to have valid data here
}
```

#### 2. Error Handling is Explicit

No more mysterious runtime errors:

```rust
enum AppError {
    NotFound,
    DatabaseError(sqlx::Error),
    ValidationError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Convert errors to HTTP responses
    }
}
```

#### 3. Middleware Composition

Axum's middleware system is powerful and type-safe:

```rust
let app = Router::new()
    .route("/", get(home))
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new())
    .layer(CorsLayer::permissive());
```

## Performance Comparison

Benchmark results (requests/second):
- **Axum**: 45,000 req/s
- **Express.js**: 8,000 req/s
- **FastAPI**: 12,000 req/s

Memory usage:
- **Axum**: 15MB
- **Express.js**: 120MB
- **FastAPI**: 85MB

## Challenges Faced

### 1. Learning Curve
The borrow checker was initially frustrating. Key insight: don't fight it, understand it.

### 2. Ecosystem Maturity
Some libraries are less mature than JavaScript/Python equivalents. But the situation is rapidly improving.

### 3. Compile Times
Longer than interpreted languages, but `cargo watch` helps during development.

## Best Practices I've Adopted

1. **Use `anyhow` for error handling**: Simplifies error propagation
2. **Leverage `serde`**: Makes JSON handling a breeze
3. **Embrace async/await**: Tokio makes async programming manageable
4. **Test everything**: Rust's testing framework is excellent

## When to Choose Rust/Axum

Perfect for:
- High-performance APIs
- Systems with strict reliability requirements
- Long-running services
- Projects where correctness is critical

Maybe not for:
- Rapid prototypes
- Simple CRUD apps
- Teams without Rust experience

## Code Example: Simple Blog Route

```rust
async fn blog_index(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let posts = load_posts()?;
    
    let mut context = Context::new();
    context.insert("posts", &posts);
    
    let html = state.tera
        .render("blog.html", &context)
        .map_err(|e| AppError::TemplateError(e))?;
    
    Ok(Html(html))
}
```

## Resources for Learning

- [Axum documentation](https://docs.rs/axum)
- [Zero to Production in Rust](https://zero2prod.com)
- [Rust Web Development](https://rustwebdevelopment.com)
- The excellent Rust community on Discord

## Conclusion

Rust with Axum isn't the right choice for every web project, but when you need performance, reliability, and type safety, it's hard to beat. The initial learning investment pays dividends in production stability and performance.

If you're curious, start with a small project. You might be surprised how productive you can be once you embrace the Rust way of thinking.

Happy coding! 🦀