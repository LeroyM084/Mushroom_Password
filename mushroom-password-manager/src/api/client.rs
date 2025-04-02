use reqwest::Client;
use serde_json::json;

const API_BASE_URL: &str = "http://localhost:5000";

#[derive(serde::Deserialize, Debug)]
pub struct PasswordResponse {
    pub password: String,
}

#[derive(Clone)]
pub struct ApiClient {
    pub client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn generate_password(&self) -> Result<String, String> {
        match self.client
            .get(&format!("{}/generate-password", API_BASE_URL))
            .send()
            .await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<PasswordResponse>().await {
                        Ok(data) => Ok(data.password),
                        Err(e) => Err(format!("Erreur de décodage JSON: {}", e)),
                    }
                } else {
                    Err(format!("Erreur HTTP lors de la génération: {}", response.status()))
                }
            }
            Err(e) => Err(format!("Erreur réseau: {}", e)),
        }
    }

    pub async fn save_password(&self, service_url: &str, password: &str, email: &str) -> Result<(), String> {
        let body = json!({
            "service": service_url,
            "password": password,
            "email": email
        });

        let response = self.client
            .post(&format!("{}/save-password", API_BASE_URL))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Erreur réseau: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Erreur HTTP: {}", response.status()))
        }
    }

    pub async fn get_saved_passwords(&self) -> Result<Vec<String>, String> {
        match self.client
            .get(&format!("{}/list-passwords", API_BASE_URL))
            .send()
            .await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<String>>().await {
                        Ok(passwords) => Ok(passwords),
                        Err(e) => Err(format!("Erreur de décodage JSON: {}", e)),
                    }
                } else {
                    Err(format!("Erreur HTTP: {}", response.status()))
                }
            }
            Err(e) => Err(format!("Erreur réseau: {}", e)),
        }
    }
}
