[![crates.io](https://img.shields.io/crates/v/spring-web.svg)](https://crates.io/crates/spring-web)
[![Documentation](https://docs.rs/spring-web/badge.svg)](https://docs.rs/spring-web)

## Dependencies

```toml
spring-web = { version = "0.0.6" }
```

## Configuration items

```toml
[web]
binding = "172.20.10.4"  # IP address of the network card to bind, default 127.0.0.1
port = 8000              # Port number to bind, default 8080

# Web middleware configuration
[web.middlewares]
compression = { enable = true }                        # Enable compression middleware
logger = { enable = true }                             # Enable log middleware
catch_panic = { enable = true }                        # Capture panic generated by handler
limit_payload = { enable = true, body_limit = "5MB" }  # Limit request body size
timeout_request = { enable = true, timeout = 60000 }   # Request timeout 60s

# Cross-domain configuration
cors = { enable = true, allow_origins = [
    "*.github.io",
], allow_headers = [
    "Authentication",
], allow_methods = [
    "GET",
    "POST",
], max_age = 60 }

# Static resource configuration
static = { enable = true, uri = "/static", path = "static", precompressed = true, fallback = "index.html" }
```

## API interface

App implements the [WebConfigurator](https://docs.rs/spring-web/latest/spring_web/trait.WebConfigurator.html) feature, which can be used to specify routing configuration:

```diff
 #[tokio::main]
 async fn main() {
    App::new()
    .add_plugin(SqlxPlugin)
    .add_plugin(WebPlugin)
+   .add_router(router())
    .run()
    .await
}

+fn router() -> Router {
+    Router::new().typed_route(hello_word)
+}

+#[get("/")]
+async fn hello_word() -> impl IntoResponse {
+   "hello word"
+}
```

You can also use the `auto_config` macro to implement automatic configuration. This process macro will automatically register the routes marked by the Procedural Macro into the app:

```diff
+#[auto_config(WebConfigurator)]
 #[tokio::main]
 async fn main() {
    App::new()
    .add_plugin(SqlxPlugin)
    .add_plugin(WebPlugin)
-   .add_router(router())
    .run()
    .await
}
```

## Attribute macro

[`get`](https://docs.rs/spring-macros/latest/spring_macros/attr.get.html) in the above example is an attribute macro. Spring provides eight standard HTTP METHOD process macros: `get`, `post`, `patch`, `put`, `delete`, `head`, `trace`, `options`.

You can also use the [`route`](https://docs.rs/spring-macros/latest/spring_macros/attr.route.html) macro to bind multiple methods at the same time:

```rust
#[route("/test", method = "GET", method = "HEAD")]
async fn example() -> impl IntoResponse {
    "hello world"
}
```

In addition, spring also supports binding multiple routes to a handler, which requires the [`routes`](https://docs.rs/spring-macros/latest/spring_macros/attr.routes.html) attribute macro:

```rust
#[routes]
#[get("/test")]
#[get("/test2")]
#[delete("/test")]
async fn example() -> impl IntoResponse {
    "hello world"
}
```

## Extract the Component registered by the plugin

In the above example, the `SqlxPlugin` plugin automatically registers a Sqlx connection pool component for us. We can use `Component` to extract this connection pool from State. [`Component`](https://docs.rs/spring-web/latest/spring_web/extractor/struct.Component.html) is an axum [extractor](https://docs.rs/axum/latest/axum/extract/index.html).

```rust
use spring::get;
use spring_sqlx::{sqlx::{self, Row}, ConnectPool};
use spring_web::extractor::Component;
use spring_web::error::Result;
use anyhow::Context;

#[get("/version")]
async fn mysql_version(Component(pool): Component<ConnectPool>) -> Result<String> {
    let version = sqlx::query("select version() as version")
        .fetch_one(&pool)
        .await
        .context("sqlx query failed")?
        .get("version");
    Ok(version)
}
```

Complete code reference [`web-example`](https://github.com/spring-rs/spring-rs/tree/master/examples/web-example)