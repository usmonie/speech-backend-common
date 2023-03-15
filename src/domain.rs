use async_trait::async_trait;
use crate::error::ApiError;

#[async_trait]
pub trait AbstractUseCase<R, T> {
    async fn execute(&self, request: R) -> Result<T, ApiError>;
}
