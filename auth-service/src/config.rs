#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub prod_ip: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET environment variable must be set"),
            prod_ip: std::env::var("PROD_IP").expect("PROD_IP environment variable must be set"),
        }
    }
}
