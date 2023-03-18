use async_trait::async_trait;

use crate::result::ApiResult;

#[async_trait]
pub trait UseCase<R, T> {
    async fn execute(&self, request: R) -> ApiResult<T>;
}
