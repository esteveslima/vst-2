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
}

pub mod application {
    pub mod interfaces {
        pub mod use_case;
    }
    pub mod use_cases {
        pub mod get_stocks_summary_use_case;
        pub mod purchase_stock_use_case;
        pub mod sell_stock_use_case;
    }
}
