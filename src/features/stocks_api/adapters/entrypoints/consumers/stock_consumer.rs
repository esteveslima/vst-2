use std::convert::Infallible;

pub struct StockConsumer;

impl StockConsumer {
    pub async fn handle_consume_test_operation(payload: String) -> Result<(), Infallible> {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("{}", payload);
        Ok(())
    }

    //...
}
