use askama::Template;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router, Server as WebServer,
};
use redis::{Client as RedisClient, Commands, ConnectionLike};
use rgm::AppError;
use serde::Deserialize;
use std::net::SocketAddr;
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rgm=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    //redis://[<username>][:<password>@]<hostname>[:port][/<db>]
    let rds_cfg =
        std::env::var("RDS").unwrap_or_else(|_| "redis://:@192.168.0.15:11111/1".to_string());
    let rds_client = RedisClient::open(rds_cfg)?;
    let mut rds_conn = rds_client.get_connection()?;
    if !rds_conn.check_connection() {
        return Err(AppError::Internal("redis connection failed".to_string()));
    }

    // build our application with some routes
    let app = Router::new()
        .route("/battle", get(get_battle).post(post_battle))
        .route("/hello", get(get_hello).post(post_hello))
        .layer(Extension(rds_client));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4040));
    tracing::debug!("listening on {}", addr);
    WebServer::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_battle(Extension(_rds): Extension<RedisClient>) -> impl IntoResponse {
    todo!()
}

async fn post_battle(Extension(_rds): Extension<RedisClient>) -> impl IntoResponse {
    todo!()
}

async fn get_hello(Extension(rds): Extension<RedisClient>) -> impl IntoResponse {
    let name = match rds.get_connection_with_timeout(Duration::from_secs(1)) {
        Ok(mut conn) => conn.get("hello").unwrap_or_default(),
        _ => "".to_string(),
    };
    let template = HelloTemplate { name };
    HtmlTemplate(template)
}

async fn post_hello(Json(hello): Json<Hello>, Extension(rds): Extension<RedisClient>) -> Response {
    match rds.get_connection_with_timeout(Duration::from_secs(1)) {
        Ok(mut conn) => {
            conn.set::<_, _, ()>("hello", hello.name).unwrap();
            StatusCode::OK.into_response()
        }
        Err(e) => AppError::Redis(e).into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct Hello {
    name: String,
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}
struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
