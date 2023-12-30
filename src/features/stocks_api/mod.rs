// It's required to declare the entire module tree,
// This is being done at every feature/module level to be easily used in the main file

pub mod adapters {
    pub mod entrypoints {
        pub mod controllers {
            pub mod stock_order_controller;
            pub mod dtos {
                pub mod get_stocks_summary_rest_dto;
                pub mod purchase_stock_rest_dto;
                pub mod sell_stock_rest_dto;
            }
        }
        pub mod models {
            pub mod api_response;
        }
    }
    pub mod gateways {
        pub mod producers {
            pub mod stock_order_producer_gateway_impl;
        }
    }
}

pub mod application {
    pub mod interfaces {
        pub mod gateways {
            pub mod producers {
                pub mod stock_order_producer_gateway;
            }
        }
    }
    pub mod use_cases {
        pub mod get_stocks_summary_use_case;
        pub mod purchase_stock_use_case;
        pub mod sell_stock_use_case;
    }
}

pub mod infrastructure {
    pub mod factories {
        pub mod stock_entrypoints_factory;
        pub mod stock_gateways_factory;
        pub mod stock_use_cases_factory;
    }
    pub mod routers {
        pub mod stock_controller_router;
    }
}

pub mod domain {
    pub mod entities {
        pub mod stock_order;
    }
}
