use async_trait::async_trait;

use crate::{
    features::stocks_api::{
        application::interfaces::gateways::producers::stock_producer_gateway::{
            ProduceStockOrderParametersDTO, ProduceStockOrderPayloadParametersDTO,
            ProduceStockOrderResultDTO, StockOrderProducerGateway,
            StockOrderProducerGatewayConstructor,
        },
        domain::entities::stock_order::StockOrder,
    },
    infrastructure::clients::stream::stream_producer_client::{
        StreamProducerClient, StreamProducerClientBuildParametersDTO,
        StreamProducerClientConstructor, StreamProducerClientImpl,
        StreamProducerClientProduceParametersDTO,
    },
};

pub struct StockOrderProducerGatewayImpl {
    stock_order_producer_client: StreamProducerClientImpl,
}

//  //  //

impl<'a> StockOrderProducerGatewayConstructor for StockOrderProducerGatewayImpl {
    fn new() -> Self {
        StockOrderProducerGatewayImpl {
            stock_order_producer_client: StreamProducerClientConstructor::new(
                StreamProducerClientBuildParametersDTO {
                    broker_host: std::env::var("KAFKA_BROKER_HOST").unwrap(),
                    topic: std::env::var("KAFKA_TOPIC_STOCK_ORDER").unwrap(),
                },
            ),
        }
    }
}

#[async_trait]
impl StockOrderProducerGateway for StockOrderProducerGatewayImpl {
    async fn produce_stock_order(
        &self,
        params: ProduceStockOrderParametersDTO,
    ) -> Result<ProduceStockOrderResultDTO, Box<dyn std::error::Error + Send + Sync>> {
        let ProduceStockOrderParametersDTO {
            user_id,
            payload:
                ProduceStockOrderPayloadParametersDTO {
                    operation,
                    shares,
                    stock,
                },
        } = params;

        let key = Some(user_id.clone());
        let stock_order = StockOrder {
            user_id,
            operation,
            shares,
            stock,
        };

        let produce_result = self
            .stock_order_producer_client
            .produce(StreamProducerClientProduceParametersDTO {
                payload: stock_order,
                optional_key: key,
            })
            .await?; //TODO: create custom errors(also, look into anyhow)

        Ok(ProduceStockOrderResultDTO {
            id: produce_result.id,
        })
    }
}