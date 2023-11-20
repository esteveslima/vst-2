use async_trait::async_trait;

#[async_trait]
pub trait UseCase<P, R> {
    async fn execute(params: P) -> Result<R, Box<dyn std::error::Error>>;
}
