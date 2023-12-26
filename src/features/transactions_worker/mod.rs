// It's required to declare the entire module tree,
// This is being done at every feature/module level to be easily used in the main file

pub mod adapters {
    pub mod entrypoints {
        pub mod consumers {
            pub mod dtos {
                pub mod stock_order_stream_consumer_data_dto;
            }
            pub mod stock_order_consumer;
        }
    }
    pub mod gateways {
        pub mod http {
            pub mod dtos {
                pub mod nasdaq_api_fetch_stock_data_dto;
            }
            pub mod stock_market_http_api_gateway_impl;
        }
        pub mod producers {
            pub mod stock_order_transaction_producer_gateway_impl;
        }
    }
}

pub mod application {
    pub mod interfaces {
        pub mod gateways {
            pub mod http {
                pub mod stock_market_http_api_gateway;
            }
            pub mod producers {
                pub mod stock_order_transaction_producer_gateway;
            }
        }
    }
    pub mod use_cases {
        pub mod create_stock_order_transaction_use_case;
    }
}

pub mod infrastructure {
    pub mod factories {
        pub mod transaction_entrypoints_factory;
        pub mod transaction_gateways_factory;
        pub mod transaction_use_cases_factory;
    }
    pub mod routers {
        pub mod transaction_consumer_router;
    }
}

pub mod domain {
    pub mod entities {
        pub mod stock_order_transaction;
    }
}
