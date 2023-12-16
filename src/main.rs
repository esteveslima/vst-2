use features::stocks_api::infrastructure::builders::{
    stock_entrypoints_builder::{StockEntrypoints, StockEntrypointsBuilder},
    stock_gateways_builder::{StockGateways, StockGatewaysBuilder},
    stock_use_cases_builder::{StockUseCases, StockUseCasesBuilder},
};
use infrastructure::{
    configurations::env_loader,
    runners::{stream_consumer_runner, web_server_runner},
};

pub mod features {
    pub mod stocks_api;
}

pub mod infrastructure {
    pub mod configurations {
        pub mod env_loader;
    }

    pub mod runners {
        pub mod stream_consumer_runner;
        pub mod web_server_runner;
    }
    pub mod stream {
        pub mod client {
            pub mod stream_consumer_client;
            pub mod stream_producer_client;
        }
    }
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref STOCK_FEATURE_GATEWAYS_INSTANCES: StockGateways<'static> = StockGateways::build();
    static ref STOCK_FEATURE_USE_CASES_INSTANCES: StockUseCases<'static> =
        StockUseCases::build(&STOCK_FEATURE_GATEWAYS_INSTANCES);
    static ref STOCK_FEATURE_ENTRYPOINTS_INSTANCES: StockEntrypoints<'static> =
        StockEntrypoints::build(&STOCK_FEATURE_USE_CASES_INSTANCES);
}

#[tokio::main]
async fn main() {
    env_loader::setup_env_config();

    let web_server_runner = web_server_runner::setup_web_server_runner(
        &STOCK_FEATURE_ENTRYPOINTS_INSTANCES.stock_controller,
    );
    let stream_consumer_runner = stream_consumer_runner::setup_stream_consumer_runner(
        &STOCK_FEATURE_ENTRYPOINTS_INSTANCES.stock_consumer,
    );

    tokio::join!(web_server_runner, stream_consumer_runner);
}
