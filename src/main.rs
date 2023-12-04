use infrastructure::{
    configurations::{
        stream_consumer_runner::{StreamConsumerRunner, StreamConsumerRunnerTrait},
        env::{Env, EnvTrait},
    },
    stream::client::stream_producer_client::{
        StreamProducerClient, StreamProducerClientSetupParameters, StreamProducerClientTrait,
    },
    web::web_server::{WebServer, WebServerTrait},
};

pub mod infrastructure {
    pub mod configurations {
        pub mod stream_consumer_runner;
        pub mod env;
    }
    pub mod web {
        pub mod web_server;
    }
    pub mod stream {
        pub mod client {
            pub mod stream_consumer_client;
            pub mod stream_producer_client;
        }
    }
}
pub mod features {
    pub mod stocks_api;
}
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    Env::setup();

    let test_broker = std::env::var("STOCK_KAFKA_BROKER_HOST").unwrap_or("".to_string());
    let test_topic = std::env::var("STOCK_KAFKA_TOPIC").unwrap_or("".to_string());
    let stream_producer_client = StreamProducerClient::setup(StreamProducerClientSetupParameters {
        broker_host: test_broker.clone(),
        topic: test_topic.clone(),
    });
    #[derive(Deserialize, Serialize, Debug, Clone)]
    struct TestPayload {
        abc: usize,
    }
    let test_payload = TestPayload { abc: 123 };
    let test_key = "some_key".to_string();
    let _stream_send_result = stream_producer_client
        .produce(test_payload.clone(), Some(test_key.clone()))
        .await;

    let web_server = WebServer::setup();
    let consumer_runner = StreamConsumerRunner::setup();

    tokio::join!(web_server, consumer_runner);
}
