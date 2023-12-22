use async_trait::async_trait;
use serde::Serialize;

use crate::{
    features::stocks_api::application::interfaces::gateways::producers::stock_producer_gateway::{
        OrderPayloadDTO, ProducePurchaseStockOrderParametersDTO,
        ProduceSellStockOrderParametersDTO, StockProducerGateway, StockProducerGatewayConstructor,
    },
    infrastructure::clients::stream::stream_producer_client::{
        StreamProducerClient, StreamProducerClientBuildParameters, StreamProducerClientConstructor,
        StreamProducerClientImpl,
    },
};

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum OrderOperation {
    PURCHASE,
    SELL,
}

#[derive(Serialize)]
struct StockOrder {
    operation: OrderOperation,
    stock: String,
    shares: usize,
}

//  //  //

pub struct StockProducerGatewayImpl {
    stock_order_producer_client: StreamProducerClientImpl,
}

//  //  //

impl<'a> StockProducerGatewayConstructor for StockProducerGatewayImpl {
    fn new() -> Self {
        StockProducerGatewayImpl {
            stock_order_producer_client: StreamProducerClientConstructor::new(
                StreamProducerClientBuildParameters {
                    broker_host: std::env::var("KAFKA_BROKER_HOST").unwrap(),
                    topic: std::env::var("KAFKA_TOPIC_STOCK_ORDER").unwrap(),
                },
            ),
        }
    }
}

#[async_trait]
impl StockProducerGateway for StockProducerGatewayImpl {
    async fn produce_purchase_stock_order(
        &self,
        params: ProducePurchaseStockOrderParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ProducePurchaseStockOrderParametersDTO {
            user_id,
            payload: OrderPayloadDTO { shares, stock },
        } = params;

        let key = Some(user_id);
        let purchase_order = StockOrder {
            operation: OrderOperation::PURCHASE,
            shares,
            stock,
        };

        self.stock_order_producer_client
            .produce(purchase_order, key)
            .await?; //TODO: create custom errors(also, look into anyhow)
        Ok(())
    }
    async fn produce_sell_stock_order(
        &self,
        params: ProduceSellStockOrderParametersDTO,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ProduceSellStockOrderParametersDTO {
            user_id,
            payload: OrderPayloadDTO { shares, stock },
        } = params;

        let key = Some(user_id);
        let sell_order = StockOrder {
            operation: OrderOperation::SELL,
            shares,
            stock,
        };

        self.stock_order_producer_client
            .produce(sell_order, key)
            .await?;
        Ok(())
    }
}
