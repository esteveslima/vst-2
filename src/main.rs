use infrastructure::{
    configurations::{
        env_loader,        
    },
    runners::{
        web_server_runner,
        stream_consumer_runner,
    },
    stream::client::stream_producer_client::{
        StreamProducerClient, StreamProducerClientSetupParameters, StreamProducerClientTrait,
    }
    
};

pub mod infrastructure {
    pub mod configurations {
        pub mod env_loader;        
    }

    pub mod runners {
        pub mod web_server_runner;
        pub mod stream_consumer_runner;
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
    env_loader::setup_env_config();

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

    let web_server_runner = web_server_runner::setup_web_server_runner();
    let stream_consumer_runner = stream_consumer_runner::setup_stream_consumer_runner();

    tokio::join!(web_server_runner, stream_consumer_runner);
}
