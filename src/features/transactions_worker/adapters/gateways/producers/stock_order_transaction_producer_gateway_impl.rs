use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    common::infrastructure::clients::stream::stream_producer_client::{
        StreamProducerClient, StreamProducerClientBuildParametersDTO,
        StreamProducerClientConstructor, StreamProducerClientImpl,
        StreamProducerClientProduceParametersDTO,
    },
    features::transactions_worker::{
        application::interfaces::gateways::producers::stock_order_transaction_producer_gateway::{
            ProduceStockOrderTransactionParametersDTO,
            ProduceStockOrderTransactionParametersPayloadDTO,
            ProduceStockOrderTransactionResultDTO, StockOrderTransactionProducerGateway,
            StockOrderTransactionProducerGatewayConstructor,
        },
        domain::entities::stock_order_transaction::StockOrderTransaction,
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
    ) -> Result<ProduceStockOrderTransactionResultDTO, Box<dyn std::error::Error + Send + Sync>>
    {
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
            id: Uuid::new_v4().to_string(),
            user_id,
            status,
            operation,
            date: Utc::now(),
            shares,
            stock,
            price,
        };

        self.stock_order_transaction_producer_client
            .produce(StreamProducerClientProduceParametersDTO {
                payload: stock_order_transaction.clone(),
                optional_key: key,
            })
            .await?;

        Ok(ProduceStockOrderTransactionResultDTO {
            id: stock_order_transaction.id,
        })
    }
}
