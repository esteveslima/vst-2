use async_trait::async_trait;

use crate::{
    features::transactions_worker::{
        application::interfaces::gateways::producers::stock_order_transaction_producer_gateway::{
            ProduceStockOrderTransactionParametersDTO,
            ProduceStockOrderTransactionParametersPayloadDTO, StockOrderTransactionProducerGateway,
            StockOrderTransactionProducerGatewayConstructor,
        },
        domain::entities::stock_order_transaction::StockOrderTransaction,
    },
    infrastructure::clients::stream::stream_producer_client::{
        StreamProducerClient, StreamProducerClientBuildParametersDTO,
        StreamProducerClientConstructor, StreamProducerClientImpl,
        StreamProducerClientProduceParametersDTO,
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
                StreamProducerClientBuildParametersDTO {
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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ProduceStockOrderTransactionParametersDTO {
            user_id,
            payload:
                ProduceStockOrderTransactionParametersPayloadDTO {
                    operation,
                    price,
                    shares,
                    status,
                    stock,
                },
        } = params;

        let key = Some(user_id.clone());
        let stock_order_transaction = StockOrderTransaction {
            user_id,
            status,
            operation,
            shares,
            stock,
            price,
        };

        self.stock_order_transaction_producer_client
            .produce(StreamProducerClientProduceParametersDTO {
                payload: stock_order_transaction,
                optional_key: key,
            })
            .await?;

        Ok(())
    }
}
