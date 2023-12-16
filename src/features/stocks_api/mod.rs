// It's required to declare the entire module tree, optioned to do it at the feature level

pub mod adapters {
    pub mod entrypoints {
        pub mod controllers {
            pub mod stock_controller;
            pub mod dtos {
                pub mod get_stocks_summary_rest_dto;
                pub mod purchase_stock_rest_dto;
                pub mod sell_stock_rest_dto;
            }
        }
        pub mod consumers {
            pub mod stock_consumer;
        }
        pub mod model {
            pub mod api_response;
        }
    }
    pub mod gateways {
        pub mod producers {
            pub mod stock_producer_gateway_impl;
        }
    }
}

pub mod application {
    pub mod interfaces {
        pub mod gateways {
            pub mod stock_producer_gateway;
        }
        pub mod use_cases {
            pub mod use_case;
        }
    }
    pub mod use_cases {
        pub mod get_stocks_summary_use_case;
        pub mod purchase_stock_use_case;
        pub mod sell_stock_use_case;
        pub mod test_consume_use_case;
    }
}

pub mod infrastructure {
    pub mod builders {
        pub mod stock_entrypoints_builder;
        pub mod stock_gateways_builder;
        pub mod stock_use_cases_builder;
    }
    pub mod routers {
        pub mod stock_consumer_router;
        pub mod stock_controller_router;
    }
}
