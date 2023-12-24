use features::{
    stocks_api::infrastructure::factories::{
        stock_entrypoints_factory::{StockEntrypoints, StockEntrypointsFactory},
        stock_gateways_factory::{StockGateways, StockGatewaysFactory},
        stock_use_cases_factory::{StockUseCases, StockUseCasesFactory},
    },
    transactions_worker::infrastructure::factories::{
        transaction_entrypoints_factory::{TransactionEntrypoints, TransactionEntrypointsFactory},
        transaction_gateways_factory::{TransactionGateways, TransactionGatewaysFactory},
        transaction_use_cases_factory::{TransactionUseCases, TransactionUseCasesFactory},
    },
};
use infrastructure::{
    configurations::env::env_loader,
    runners::{
        stream_consumer_runner::{self, StreamConsumerRunnerParameters},
        web_server_runner::{self, WebServerRunnerParameters},
    },
};

pub mod application;
pub mod infrastructure;
pub mod features {
    pub mod stocks_api;
    pub mod transactions_worker;
}

#[macro_use]
extern crate lazy_static;
lazy_static! {
    // Stock instances pool
    static ref STOCK_FEATURE_GATEWAYS_INSTANCES: StockGateways<'static> = StockGateways::build();
    static ref STOCK_FEATURE_USE_CASES_INSTANCES: StockUseCases<'static> =
        StockUseCases::build(&STOCK_FEATURE_GATEWAYS_INSTANCES);
    static ref STOCK_FEATURE_ENTRYPOINTS_INSTANCES: StockEntrypoints<'static> =
        StockEntrypoints::build(&STOCK_FEATURE_USE_CASES_INSTANCES);

    // Transaction instances pool
    static ref TRANSACTION_FEATURE_GATEWAYS_INSTANCES: TransactionGateways<'static> =
        TransactionGateways::build();
    static ref TRANSACTION_FEATURE_USE_CASES_INSTANCES: TransactionUseCases<'static> =
        TransactionUseCases::build(&TRANSACTION_FEATURE_GATEWAYS_INSTANCES);
    static ref TRANSACTION_FEATURE_ENTRYPOINTS_INSTANCES: TransactionEntrypoints<'static> =
        TransactionEntrypoints::build(&TRANSACTION_FEATURE_USE_CASES_INSTANCES);
}

#[tokio::main]
async fn main() {
    env_loader::load_env();

    let web_server_runner = web_server_runner::setup_web_server_runner(WebServerRunnerParameters {
        stock_controller: &STOCK_FEATURE_ENTRYPOINTS_INSTANCES.stock_controller,
    });
    let stream_consumer_runner =
        stream_consumer_runner::setup_stream_consumer_runner(StreamConsumerRunnerParameters {
            stock_order_consumer: &TRANSACTION_FEATURE_ENTRYPOINTS_INSTANCES.stock_order_consumer,
        });

    tokio::join!(web_server_runner, stream_consumer_runner);
}

// TODOS:
//V 1 - make the api produce stock orders
//V 1.1 - add keys by user id to ensure ordering
// 2 - make a worker module to handle the orders and generate the transactions   -> fix stock names to keep pattern
// 3 - add materialize to docker environment
// 4 - create a client to connect to materialize and add materialized views
// 5 - maybe refactor to have the kafka data as entities
// 6 - create queries to fetch data from materialized the views
// 7 - search for how to create cron jobs
// 8 - implement cron jobs to watch stocks fluctuations and store data for future queries
