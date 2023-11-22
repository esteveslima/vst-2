use dotenv;
use warp::Filter;

pub mod features {
    pub mod stocks_api;
}

#[tokio::main]
async fn main() {    
    load_env();

    let stocks_controller =
        features::stocks_api::adapters::entrypoints::controllers::stock_controller::build_controller();

    let routers = warp::any().and(stocks_controller);

    warp::serve(routers).run(([0, 0, 0, 0], 3030)).await;
}

//

fn load_env() {
    let path = "assets/environment/.env";
    let env_load_result = dotenv::from_path(path);

    if env_load_result.is_err() {
        println!("No .env file found. Using default or system environment variables.");
    }
}