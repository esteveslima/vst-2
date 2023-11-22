use infrastructure::{
    configurations::env::{EnvLoader, EnvLoaderTrait},
    web::web_server::{WebServer, WebServerTrait},
};

pub mod infrastructure {
    pub mod configurations {
        pub mod env;
    }
    pub mod web {
        pub mod web_server;
    }
}
pub mod features {
    pub mod stocks_api;
}

#[tokio::main]
async fn main() {
    EnvLoader::load();

    WebServer::start().await;
}
