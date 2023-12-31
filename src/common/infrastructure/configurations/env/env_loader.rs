use dotenv;

pub fn load_env() {
    let path = "assets/environment/.env";
    let env_setup_result = dotenv::from_path(path);

    if env_setup_result.is_err() {
        println!("No .env file found. Using default or system environment variables.");
    }
}
