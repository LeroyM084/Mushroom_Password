use reqwest::Client;
use serde_json::json;
use serde::Deserialize;
use regex::Regex;
use std::collections::HashMap;

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
    #[serde(default)] // Permet une valeur par défaut si absent
    email: Option<String>, // Rend le champ optionnel
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
        // Simplifier l'URL en supprimant http/https/www
        let url = url.replace("https://", "")
                    .replace("http://", "")
                    .replace("www.", "");
        
        // Prendre uniquement la partie avant le premier '/'
        let domain_part = url.split('/').next().unwrap_or("");
        
        // Simplifié pour ne pas exiger un point
        let parts: Vec<&str> = domain_part.split('.').collect();
        
        if parts.len() >= 2 {
            // Format "subdomain.domain" (comme "compte.auchan")
            format!("{}.{}", parts[0], parts[1])
        } else if parts.len() == 1 {
            // Juste un domaine simple
            parts[0].to_string()
        } else {
            // Fallback
            domain_part.to_string()
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

    // Ajout d'une méthode pour récupérer toutes les données des mots de passe
    async fn get_data(&self) -> Result<HashMap<String, SavedPassword>, String> {
        match self.client
            .get(&format!("{}/list-passwords", API_BASE_URL))
            .send()
            .await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<HashMap<String, SavedPassword>>().await {
                        Ok(data) => Ok(data),
                        Err(e) => Err(format!("Erreur de décodage JSON: {}", e)),
                    }
                } else {
                    Err(format!("Erreur HTTP: {}", response.status()))
                }
            }
            Err(e) => Err(format!("Erreur réseau: {}", e)),
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
                                for (service_url, details) in map {
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

    // Changé de async fn privée à pub async fn publique
    pub async fn get_password_details(&self, display_name: &str) -> Result<(String, String, String), String> {
        println!("Recherche du service: {}", display_name);
        
        // Récupérer les données
        let data = self.get_data().await.map_err(|e| {
            println!("Erreur get_data: {}", e);
            e.to_string()
        })?;
        
        // Afficher toutes les entrées pour le débogage
        println!("Services disponibles:");
        for (key, details) in &data {
            println!("  - URL: {}, Nom: {}", key, details.service_name);
        }
        
        // Trouver l'URL correspondante
        let mut found_service_url = None;
        
        for (key, details) in &data {
            // D'abord, chercher une correspondance exacte
            if details.service_name == display_name {
                found_service_url = Some(key.clone());
                println!("Correspondance exacte trouvée: {}", key);
                break;
            }
        }
        
        // Si pas de correspondance exacte, chercher une correspondance partielle
        if found_service_url.is_none() {
            for (key, details) in &data {
                if details.service_name.contains(display_name) || 
                   display_name.contains(&details.service_name) {
                    found_service_url = Some(key.clone());
                    println!("Correspondance partielle trouvée: {}", key);
                    break;
                }
            }
        }
        
        let service_url = match found_service_url {
            Some(url) => url,
            None => {
                println!("Aucun service trouvé pour: {}", display_name);
                return Err(format!("Service '{}' non trouvé", display_name));
            }
        };
        
        println!("URL de service trouvée: {}", service_url);
        
        // Faire la requête à l'API pour obtenir le mot de passe
        let body = json!({
            "service_URL": service_url
        });
        
        println!("Envoi de la requête à /get-password: {:?}", body);
        
        match self.client
            .post(&format!("{}/get-password", API_BASE_URL))
            .json(&body)
            .send()
            .await {
            Ok(response) => {
                let status = response.status();
                println!("Statut de la réponse: {}", status);
                
                if status.is_success() {
                    match response.text().await {
                        Ok(text) => {
                            println!("Réponse brute: {}", text);
                            
                            match serde_json::from_str::<serde_json::Value>(&text) {
                                Ok(json_value) => {
                                    // Extraction du mot de passe et de l'email
                                    if let Some(password) = json_value.get("service_password") {
                                        if let Some(password_str) = password.as_str() {
                                            let email = json_value.get("email")
                                                .and_then(|e| e.as_str())
                                                .unwrap_or("");
                                            
                                            println!("Mot de passe et email extraits avec succès");
                                            return Ok((
                                                service_url,
                                                password_str.to_string(),
                                                email.to_string(),
                                            ));
                                        }
                                    }
                                    
                                    Err(format!("Format de réponse invalide: {}", text))
                                },
                                Err(e) => {
                                    Err(format!("Erreur de décodage JSON: {}. Réponse: {}", e, text))
                                }
                            }
                        },
                        Err(e) => Err(format!("Erreur lors de la lecture de la réponse: {}", e)),
                    }
                } else {
                    match response.text().await {
                        Ok(text) => Err(format!("Erreur HTTP {}: {}", status, text)),
                        Err(_) => Err(format!("Erreur HTTP {}", status)),
                    }
                }
            },
            Err(e) => {
                println!("Erreur lors de l'envoi de la requête: {}", e);
                Err(format!("Erreur réseau: {}", e))
            },
        }
    }

    // Fonction pour mettre à jour un mot de passe existant
    pub async fn update_password(&self, service_url: &str, password: &str, email: &str) -> Result<(), String> {
        let body = json!({
            "service": service_url,
            "password": password,
            "email": email
        });
    
        match self.client
            .post(&format!("{}/save-password", API_BASE_URL))
            .json(&body)
            .send()
            .await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    let status = response.status();
                    match response.text().await {
                        Ok(text) => Err(format!("Erreur HTTP: {}. Détails: {}", status, text)),
                        Err(_) => Err(format!("Erreur HTTP: {}", status)),
                    }
                }
            }
            Err(e) => Err(format!("Erreur réseau: {}", e)),
        }
    }
}