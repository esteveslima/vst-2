use async_trait::async_trait;

use crate::{
    features::transactions_worker::application::interfaces::gateways::producers::stock_order_transaction_producer_gateway::{
            StockOrderTransactionProducerGatewayConstructor, StockOrderTransactionProducerGateway, ProduceStockOrderTransactionParametersDTO, ProduceStockOrderTransactionParametersPayloadDTO
    },
    infrastructure::clients::stream::stream_producer_client::{
        StreamProducerClient, StreamProducerClientBuildParameters, StreamProducerClientConstructor,
        StreamProducerClientImpl,
    },
};

//  //  //

pub struct StockOrderTransactionProducerGatewayImpl {
    stock_order_transaction_producer_client: StreamProducerClientImpl,
}

//  //  //

impl<'a> StockOrderTransactionProducerGatewayConstructor
    for StockOrderTransactionProducerGatewayImpl
{
    fn new() -> Self {
        StockOrderTransactionProducerGatewayImpl {
            stock_order_transaction_producer_client: StreamProducerClientConstructor::new(
                StreamProducerClientBuildParameters {
                    broker_host: std::env::var("KAFKA_BROKER_HOST").unwrap(),
                    topic: std::env::var("KAFKA_TOPIC_STOCK_ORDER_TRANSACTION").unwrap(),
                },
            ),
        }
    }
}

#[async_trait]
impl StockOrderTransactionProducerGateway for StockOrderTransactionProducerGatewayImpl {
    async fn produce_stock_order_transaction(
        &self,
        params: ProduceStockOrderTransactionParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ProduceStockOrderTransactionParametersDTO {
            user_id,
            payload
        } = params;

        let key = Some(user_id);

        self.stock_order_transaction_producer_client
            .produce(payload, key)
            .await?;

        Ok(())
    }
}
