use async_trait::async_trait;

#[async_trait]
pub trait UseCase<P, R>: Sync + Send {
    async fn execute(&self, params: P) -> Result<R, Box<dyn std::error::Error>>;
}
