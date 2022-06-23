#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("internal error: {0}")]
    Internal(String),
    #[error("http status: {0}")]
    HttpCode(u16),
}
