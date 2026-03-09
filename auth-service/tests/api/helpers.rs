use auth_service::Application;
use reqwest::Client;
use serde::Serialize;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = Client::new();

        Self {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn signup(&self, payload: impl Serialize) -> reqwest::Response {
        self.http_client
            .post(format!("{}/signup", &self.address))
            .json(&payload)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn login(&self, payload: impl Serialize) -> reqwest::Response {
        self.http_client
            .post(format!("{}/login", &self.address))
            .json(&payload)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_2fa(&self, payload: impl Serialize) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .json(&payload)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_token(&self, payload: impl Serialize) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(&payload)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
