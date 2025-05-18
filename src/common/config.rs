use std::env;

pub struct Config {
    pub table_name: String,
}

impl Config {
    pub fn load_from_env() -> Self {
        Self {
            table_name: env::var("TABLE_NAME")
                .expect("TABLE_NAME environment variable is not set"),
        }
    }
}
