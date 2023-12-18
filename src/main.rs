use features::stocks_api::infrastructure::factories::{
    stock_entrypoints_factory::{StockEntrypoints, StockEntrypointsFactory},
    stock_gateways_factory::{StockGateways, StockGatewaysFactory},
    stock_use_cases_factory::{StockUseCases, StockUseCasesFactory},
};
use infrastructure::{
    configurations::env::env_loader,
    runners::{
        stream_consumer_runner::{self, StreamConsumerRunnerParameters},
        web_server_runner::{self, WebServerRunnerParameters},
    },
};

pub mod infrastructure;
pub mod features {
    pub mod stocks_api;
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
    env_loader::load_env();

    let web_server_runner = web_server_runner::setup_web_server_runner(WebServerRunnerParameters {
        stock_controller: &STOCK_FEATURE_ENTRYPOINTS_INSTANCES.stock_controller,
    });
    let stream_consumer_runner =
        stream_consumer_runner::setup_stream_consumer_runner(StreamConsumerRunnerParameters {
            stock_consumer: &STOCK_FEATURE_ENTRYPOINTS_INSTANCES.stock_consumer,
        });

    tokio::join!(web_server_runner, stream_consumer_runner);
}
