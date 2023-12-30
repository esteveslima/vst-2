// It's required to declare the entire module tree,
// This is being done at every feature/module level to be easily used in the main file

pub mod application {
    pub mod interfaces {
        pub mod use_case;
    }
}

pub mod infrastructure {
    pub mod configurations {
        pub mod env {
            pub mod env_loader;
        }
    }

    pub mod runners {
        pub mod stream_consumer_runner;
        pub mod web_server_runner;
    }
    pub mod clients {
        pub mod stream {
            pub mod stream_consumer_client;
            pub mod stream_producer_client;
        }
    }
}
