use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub async fn from_env() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            Config {database_url}

    }
}
