use async_trait::async_trait;

use crate::{
    features::stocks_api::application::interfaces::gateways::stock_producer_gateway::{
        PurchaseStockEventParametersDTO, SellStockEventParametersDTO, StockProducerGateway,
        StockProducerGatewayConstructor,
    },
    infrastructure::clients::stream::stream_producer_client::{
        StreamProducerClient, StreamProducerClientBuildParameters, StreamProducerClientConstructor,
        StreamProducerClientImpl,
    },
};

pub struct StockProducerGatewayImpl {
    stream_producer_client: StreamProducerClientImpl,
}

impl<'a> StockProducerGatewayConstructor for StockProducerGatewayImpl {
    fn new() -> Self {
        StockProducerGatewayImpl {
            stream_producer_client: StreamProducerClientConstructor::new(
                StreamProducerClientBuildParameters {
                    broker_host: std::env::var("STOCK_KAFKA_BROKER_HOST").unwrap(),
                    topic: std::env::var("STOCK_KAFKA_TOPIC").unwrap(),
                },
            ),
        }
    }
}

#[async_trait]
impl StockProducerGateway for StockProducerGatewayImpl {
    async fn produce_event_purchase_stock(
        &self,
        params: PurchaseStockEventParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = None;
        self.stream_producer_client.produce(params, key).await?;
        Ok(())
    }
    async fn produce_event_sell_stock(
        &self,
        params: SellStockEventParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = None;
        self.stream_producer_client.produce(params, key).await?;
        Ok(())
    }
}
