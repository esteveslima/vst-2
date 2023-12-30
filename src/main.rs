use common::infrastructure::{
    configurations::env::env_loader,
    runners::{
        stream_consumer_runner::{self, StreamConsumerRunnerInstances},
        web_server_runner::{self, WebServerRunnerInstances},
    },
};
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

pub mod common;
pub mod features {
    pub mod stocks_api;
    pub mod transactions_worker;
}

// Creating static instances to avoid lifetime issues
#[macro_use]
extern crate lazy_static;
lazy_static! {
    // stocks_api instances pool
    static ref STOCKS_API_GATEWAYS_INSTANCES: StockGateways<'static> = {
        let handle = tokio::runtime::Handle::current();
        std::thread::spawn(move || {
            handle.block_on(async {
                StockGateways::build().await
            })
        }).join().unwrap()
    };
    static ref STOCKS_API_USE_CASES_INSTANCES: StockUseCases<'static> =
        StockUseCases::build(&STOCKS_API_GATEWAYS_INSTANCES);
    static ref STOCKS_API_ENTRYPOINTS_INSTANCES: StockEntrypoints<'static> =
        StockEntrypoints::build(&STOCKS_API_USE_CASES_INSTANCES);

    // transactions_worker instances pool
    static ref TRANSACTIONS_WORKER_GATEWAYS_INSTANCES: TransactionGateways<'static> =
        TransactionGateways::build();
    static ref TRANSACTIONS_WORKER_USE_CASES_INSTANCES: TransactionUseCases<'static> =
        TransactionUseCases::build(&TRANSACTIONS_WORKER_GATEWAYS_INSTANCES);
    static ref TRANSACTIONS_WORKER_ENTRYPOINTS_INSTANCES: TransactionEntrypoints<'static> =
        TransactionEntrypoints::build(&TRANSACTIONS_WORKER_USE_CASES_INSTANCES);
}

#[tokio::main]
async fn main() {
    env_loader::load_env();

    let web_server_runner = web_server_runner::setup_web_server_runner(WebServerRunnerInstances {
        stock_controller: &STOCKS_API_ENTRYPOINTS_INSTANCES.stock_controller,
    });
    let stream_consumer_runner =
        stream_consumer_runner::setup_stream_consumer_runner(StreamConsumerRunnerInstances {
            stock_order_consumer: &TRANSACTIONS_WORKER_ENTRYPOINTS_INSTANCES.stock_order_consumer,
        });

    tokio::join!(web_server_runner, stream_consumer_runner);
}

// TODO:
//V 1 - make the api produce stock orders
//V 1.1 - add keys by user id to ensure ordering
//V 2 - make a worker module to handle the orders and generate the transactions   -> fix stock names to keep pattern
//V 3 - add materialize to docker environment
//V 4 - create a client to connect to materialize and add materialized views
//V 5 - maybe refactor to have the kafka data as entities
//V 5.1 - reevaluate to add them as simple producer models
//V 6 - create queries to fetch data from materialized the views
//V 6.1 - move common files into a "common" folder
// 6.1.0 - replace the mock users
// 6.1.1 - maybe decouple structs DTOs from code files
// 6.2 - create graphql endpoints mirroring the rest ones
// 7 - search for how to create cron jobs
// 8 - implement cron jobs to watch stocks fluctuations and store data for future queries
// 9 - upgrade to lts an remove async_trait and modify returns to use impl instead of dyn
// 10 - maybe replace lazy static with the new oncelock
