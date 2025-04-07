use reqwest::Client;
use serde_json::json;
use serde::Deserialize;
use regex::Regex;

const API_BASE_URL: &str = "http://localhost:5000";

#[derive(Deserialize, Debug)]
pub struct PasswordResponse {
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct EmailResponse {
    pub email: String,
}

#[derive(Deserialize, Debug)]
struct SavedPassword {
    service_URL: String,
    service_name: String,
    service_password: String,
}

#[derive(Clone, Debug)]
pub struct ApiClient {
    pub client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient {
            client: Client::new(),
        }
    }

    // Fonction pour extraire le nom du service à partir d'une URL
    pub fn extract_service_name(&self, url: &str) -> String {
        // Regex modifiée sans utiliser d'assertions anticipées (lookahead)
        let domain_regex = Regex::new(r"https?://(?:www\.)?(?:[\w\-]+\.)?([\w\-]+)\.\w{2,3}/?").unwrap();
        
        match domain_regex.captures(url) {
            Some(captures) => {
                if let Some(domain_match) = captures.get(1) {
                    let domain = domain_match.as_str();
                    // Met la première lettre en majuscule et le reste en minuscule
                    let mut chars = domain.chars();
                    match chars.next() {
                        Some(first_char) => {
                            let capitalized = first_char.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase();
                            return capitalized;
                        }
                        None => return String::from("Service inconnu"),
                    }
                }
                    String::from("Service inconnu")
                }
                None => String::from("Service inconnu"),
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
        // Extraction du nom de service
        let service_name = self.extract_service_name(service_url);
        
        let body = json!({
            "service": service_url,
            "service_name": service_name,  // Ajout du nom de service extrait
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
                    // D'abord, récupérons la réponse sous forme d'objet JSON générique
                    match response.json::<serde_json::Value>().await {
                        Ok(json_value) => {
                            // Maintenant, transformons cet objet en une liste de chaînes
                            let mut passwords = Vec::new();
                            
                            // Si c'est un objet (map), nous extrayons les clés
                            if let serde_json::Value::Object(map) = json_value {
                                for (service_url, _details) in map {
                                    // On peut aussi extraire le nom du service ici si besoin
                                    let service_name = self.extract_service_name(&service_url);
                                    passwords.push(format!("{}", service_name));
                                }
                                Ok(passwords)
                            } else {
                                // Si par hasard c'est déjà une liste, essayons de l'utiliser directement
                                match serde_json::from_value::<Vec<String>>(json_value) {
                                    Ok(list) => Ok(list),
                                    Err(_) => Err("Format de réponse inattendu".to_string())
                                }
                            }
                        },
                        Err(e) => Err(format!("Erreur de décodage JSON: {}", e)),
                    }
                } else {
                    Err(format!("Erreur HTTP: {}", response.status()))
                }
            }
            Err(e) => Err(format!("Erreur réseau: {}", e)),
        }
    }
    pub async fn get_password_details(&self, service_name: &str) -> Result<(String, String, String), String> {
        // Récupérer d'abord tous les services pour trouver l'URL correspondant au nom
        match self.client
            .get(&format!("{}/list-passwords", API_BASE_URL))
            .send()
            .await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<serde_json::Value>().await {
                        Ok(json_value) => {
                            if let serde_json::Value::Object(map) = json_value {
                                // Chercher le service par son nom
                                for (service_url, details) in map {
                                    if let serde_json::Value::Object(details_map) = details {
                                        if let Some(serde_json::Value::String(name)) = details_map.get("service_name") {
                                            if name == service_name {
                                                // Trouvé, maintenant récupérer le mot de passe déchiffré
                                                let body = json!({
                                                    "service_URL": service_url
                                                });
                                                
                                                let password_response = self.client
                                                    .post(&format!("{}/get-password", API_BASE_URL))
                                                    .json(&body)
                                                    .send()
                                                    .await
                                                    .map_err(|e| format!("Erreur réseau: {}", e))?;
                                                    
                                                if password_response.status().is_success() {
                                                    let pwd_data = password_response.json::<serde_json::Value>().await
                                                        .map_err(|e| format!("Erreur de décodage: {}", e))?;
                                                    
                                                    let password = pwd_data.get("service_password")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("").to_string();
                                                    
                                                    let email = details_map.get("email")
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("").to_string();
                                                        
                                                    return Ok((service_url, password, email));
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(format!("Service '{}' non trouvé", service_name))
                            } else {
                                Err("Format de réponse inattendu".to_string())
                            }
                        },
                        Err(e) => Err(format!("Erreur de décodage JSON: {}", e)),
                    }
                } else {
                    Err(format!("Erreur HTTP: {}", response.status()))
                }
            }
            Err(e) => Err(format!("Erreur réseau: {}", e)),
        }
    }
    
    pub async fn update_password(&self, service_url: &str, password: &str, email: &str) -> Result<(), String> {
        // Utiliser la même fonction que pour sauvegarder un mot de passe
        // Le backend remplacera les informations si le service existe déjà
        self.save_password(service_url, password, email).await
    }
    pub async fn get_email(&self) -> Result<String, String> {
        match self.client
            .get(&format!("{}/getEmail", API_BASE_URL))
            .send()
            .await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<EmailResponse>().await {
                        Ok(email_data) => Ok(email_data.email),
                        Err(e) => Err(format!("Erreur de décodage JSON pour l'email: {}", e)),
                    }
                } else {
                    Err(format!("Erreur HTTP lors de la récupération de l'email: {}", response.status()))
                }
            }
            Err(e) => Err(format!("Erreur réseau lors de la récupération de l'email: {}", e)),
        }
    }

    pub async fn save_email(&self, email: &str) -> Result<(), String> {
        let body = json!({
            "email": email,
        });

        let response = self.client
            .post(&format!("{}/changeMail", API_BASE_URL))
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
}