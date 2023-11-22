use dotenv;

pub trait EnvLoaderTrait {
    fn load();
}

pub struct EnvLoader;

impl EnvLoaderTrait for EnvLoader {
    fn load() {
        let path = "assets/environment/.env";
        let env_load_result = dotenv::from_path(path);

        if env_load_result.is_err() {
            println!("No .env file found. Using default or system environment variables.");
        }
    }
}
