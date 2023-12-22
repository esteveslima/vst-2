use async_trait::async_trait;
use serde::Serialize;

use crate::{
    features::transactions_worker::application::interfaces::gateways::producers::stock_order_transaction_producer_gateway::{StockOrderTransactionProducerGatewayConstructor, ProduceStockOrderTransactionParametersDTO, StockOrderTransactionPayloadDTO, StockOrderTransactionProducerGateway},
    infrastructure::clients::stream::stream_producer_client::{
        StreamProducerClient, StreamProducerClientBuildParameters, StreamProducerClientConstructor,
        StreamProducerClientImpl,
    },
};

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum StockOrderTransactionStatus {
    SUCCESSFUL,
    FAILED,
}

#[derive(Serialize)]
struct StockOrderTransaction {
    status: StockOrderTransactionStatus,
    stock: String,
    shares: usize,
    price: usize,
}

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
    async fn produce_successful_stock_order_transaction(
        &self,
        params: ProduceStockOrderTransactionParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ProduceStockOrderTransactionParametersDTO {
            user_id,
            payload:
                StockOrderTransactionPayloadDTO {
                    shares,
                    stock,
                    price,
                },
        } = params;

        let key = Some(user_id);
        let stock_order_transaction = StockOrderTransaction {
            status: StockOrderTransactionStatus::SUCCESSFUL,
            shares,
            stock,
            price,
        };

        self.stock_order_transaction_producer_client
            .produce(stock_order_transaction, key)
            .await?;

        Ok(())
    }
    async fn produce_failed_stock_order_transaction(
        &self,
        params: ProduceStockOrderTransactionParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ProduceStockOrderTransactionParametersDTO {
            user_id,
            payload:
                StockOrderTransactionPayloadDTO {
                    shares,
                    stock,
                    price,
                },
        } = params;

        let key = Some(user_id);
        let stock_order_transaction = StockOrderTransaction {
            status: StockOrderTransactionStatus::FAILED,
            shares,
            stock,
            price,
        };

        self.stock_order_transaction_producer_client
            .produce(stock_order_transaction, key)
            .await?;

        Ok(())
    }
}
