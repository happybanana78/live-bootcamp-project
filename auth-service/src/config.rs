#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET environment variable must be set"),
        }
    }
}
