use iced::{
    Application, Command, Element, Length, Settings, Subscription, Theme,
    widget::{Button, Column, Container, Row, Scrollable, Text, TextInput},
};
use iced::clipboard;

use crate::api::client::ApiClient;

// Définition des différentes vues de l'application
#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Main,
    ServiceDetail,
}

// Messages d'application
#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    ServiceUrlChanged(String),
    GeneratePassword,
    CopyToClipboard,
    SavePassword,
    RefreshPasswords,
    EmailInputChanged(String),
    SaveEmail,
    EmailReceived(String),
    PasswordSelected(String),
    PasswordGenerated(String),
    PasswordsUpdated(Vec<String>),
    StatusUpdate(String),
    GetEmail,
    
    // Nouveaux messages
    ServiceSelected(String),
    PasswordDetailsReceived(String, String, String), // URL, password, email
    ToggleEditMode,
    UpdatePassword,
    ClearForm,
    
    // Ajoutez cette ligne ici
    ClearFormFields,
    
    // Nouveaux messages pour la navigation
    NavigateTo(View),
    BackToMain,
}

pub struct PasswordManagerApp {
    // Vue actuelle
    current_view: View,
    
    // Valeurs des champs
    password_value: String,
    service_url_value: String,
    email_value: String,

    // État de l'application
    api_client: ApiClient,
    passwords: Vec<String>,
    selected_password: Option<String>,
    
    // Nouveaux états
    editing_mode: bool,
    current_service_url: Option<String>,

    // Messages système
    status_message: Option<String>,
}

impl Application for PasswordManagerApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                current_view: View::Main,
                password_value: String::new(),
                service_url_value: String::new(),
                email_value: String::new(),
                api_client: ApiClient::new(),
                passwords: Vec::new(),
                selected_password: None,
                editing_mode: false,
                current_service_url: None,
                status_message: None,
            },
            Command::batch(vec![
                Command::perform(async { Message::GetEmail }, |_| Message::GetEmail),
                Command::perform(async { Message::RefreshPasswords }, |msg| msg),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Gestionnaire de mots de passe Mushroom")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => {
                self.password_value = value;
                Command::none()
            }
            Message::ServiceUrlChanged(value) => {
                self.service_url_value = value;
                Command::none()
            }
            Message::GeneratePassword => {
                let client = self.api_client.clone();
                return Command::perform(
                    async move { client.generate_password().await },
                    |result| match result {
                        Ok(password) => Message::PasswordGenerated(password),
                        Err(e) => Message::StatusUpdate(format!("Erreur génération: {}", e)),
                    },
                );
            }
            Message::CopyToClipboard => {
                let command = clipboard::write::<Message>(self.password_value.clone());
                self.status_message = Some(String::from("Mot de passe copié dans le presse-papiers"));
                return command;
            }
            Message::SavePassword => {
                let client = self.api_client.clone();
                let service_url = self.service_url_value.clone();
                let password = self.password_value.clone();
                let email = self.email_value.clone();

                return Command::perform(
                    async move { client.save_password(&service_url, &password, &email).await },
                    |result| match result {
                        Ok(_) => {
                            Command::perform(
                                async { Message::RefreshPasswords }, 
                                |msg| msg
                            );
                            Message::StatusUpdate("Sauvegarde réussie".into())
                        },
                        Err(e) => Message::StatusUpdate(format!("Erreur: {}", e)),
                    },
                );
            }
            Message::RefreshPasswords => {
                let client = self.api_client.clone();
                return Command::perform(
                    async move { client.get_saved_passwords().await },
                    |result| match result {
                        Ok(passwords) => Message::PasswordsUpdated(passwords),
                        Err(e) => Message::StatusUpdate(format!("Erreur: {}", e)),
                    },
                );
            }
            Message::EmailInputChanged(value) => {
                self.email_value = value;
                Command::none()
            }
            Message::SaveEmail => {
                let client = self.api_client.clone();
                let email = self.email_value.clone();
                return Command::perform(
                    async move { client.save_email(&email).await },
                    |result| match result {
                        Ok(_) => Message::StatusUpdate("Email sauvegardé!".into()),
                        Err(e) => Message::StatusUpdate(format!("Erreur sauvegarde email: {}", e)),
                    },
                );
            }
            Message::PasswordSelected(password) => {
                self.selected_password = Some(password);
                Command::none()
            }
            Message::PasswordGenerated(password) => {
                self.password_value = password;
                Command::none()
            }
            Message::PasswordsUpdated(passwords) => {
                self.passwords = passwords;
                Command::none()
            }
            Message::StatusUpdate(status) => {
                self.status_message = Some(status);
                Command::none()
            }
            Message::GetEmail => {
                let client = self.api_client.clone();
                return Command::perform(
                    async move { client.get_email().await },
                    |result| match result {
                        Ok(email) => Message::EmailReceived(email),
                        Err(e) => Message::StatusUpdate(format!("Erreur: {}", e)),
                    },
                );
            }
            Message::EmailReceived(email) => {
                self.email_value = email;
                Command::none()
            }
            
            // Nouveaux handlers de messages
            Message::ServiceSelected(service_name) => {
                // On définit le service sélectionné
                self.selected_password = Some(service_name.clone());
                let service_name_clone = service_name.clone();
                let client = self.api_client.clone();
                
                // On récupère d'abord les détails du mot de passe
                let cmd = Command::perform(
                    async move { client.get_password_details(&service_name_clone).await },
                    move |result| match result {
                        Ok((service_url, password, email)) => {
                            Message::PasswordDetailsReceived(service_url, password, email)
                        },
                        Err(e) => {
                            Message::StatusUpdate(format!("Erreur: Service '{}' non trouvé. {}", service_name, e))
                        },
                    },
                );
                
                // On navigue vers la vue détaillée
                self.current_view = View::ServiceDetail;
                cmd
            }
            
            Message::PasswordDetailsReceived(service_url, password, email) => {
                self.service_url_value = service_url.clone();
                self.password_value = password;
                self.email_value = email;
                self.current_service_url = Some(service_url);
                self.editing_mode = true;
                self.status_message = Some(format!("Informations du service chargées"));
                Command::none()
            }
            
            Message::ToggleEditMode => {
                self.editing_mode = !self.editing_mode;
                Command::none()
            }
            
            Message::UpdatePassword => {
                if let Some(service_url) = &self.current_service_url {
                    let client = self.api_client.clone();
                    let service_url = service_url.clone();
                    let password = self.password_value.clone();
                    let email = self.email_value.clone();
                    
                    self.status_message = Some(format!("Mise à jour en cours..."));
                    
                    return Command::perform(
                        async move { 
                            println!("Mise à jour du service: {}, email: {}", service_url, email);
                            client.update_password(&service_url, &password, &email).await 
                        },
                        |result| match result {
                            Ok(_) => Message::StatusUpdate("Mot de passe mis à jour avec succès".into()),
                            Err(e) => Message::StatusUpdate(format!("Erreur de mise à jour: {}", e)),
                        },
                    );
                } else {
                    self.status_message = Some(String::from("Erreur: URL du service non disponible"));
                }
                Command::none()
            }
            
            Message::ClearForm => {
                self.password_value = String::new();
                self.service_url_value = String::new();
                self.current_service_url = None;
                self.editing_mode = false;
                self.selected_password = None;
                Command::none()
            }
            
            // Navigation
            Message::NavigateTo(view) => {
                self.current_view = view;
                Command::none()
            }
            
            Message::BackToMain => {
                // Réinitialiser l'état et retourner à la vue principale
                self.current_view = View::Main;
                self.selected_password = None;
                self.editing_mode = false;
                self.current_service_url = None;
                
                // Vider les champs du formulaire
                self.password_value = String::new();
                self.service_url_value = String::new();
                self.email_value = String::new();
                
                // Rafraîchir la liste des mots de passe
                return Command::perform(
                    async { Message::RefreshPasswords },
                    |msg| msg
                );
            }
            Message::ClearForm => {
                self.password_value = String::new();
                self.service_url_value = String::new();
                self.current_service_url = None;
                self.editing_mode = false;
                self.selected_password = None;
                Command::none()
            },
            
            // Ajoutez ce gestionnaire ici
            Message::ClearFormFields => {
                self.password_value = String::new();
                self.service_url_value = String::new();
                self.email_value = String::new();
                Command::none()
            },
            
            // Navigation
            Message::NavigateTo(view) => {
                self.current_view = view;
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<Message> {
        match self.current_view {
            View::Main => self.view_main(),
            View::ServiceDetail => self.view_service_detail(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

impl PasswordManagerApp {
    // Vue principale avec la liste des services et le formulaire d'ajout
    fn view_main(&self) -> Element<Message> {
        // Liste des mots de passe sauvegardés
        let mut passwords_list = Column::new().spacing(10);
    
        for password in &self.passwords {
            let is_selected = self.selected_password.as_ref().map_or(false, |s| s == password);
            
            let password_button = Button::new(
                Text::new(password)
                    .size(if is_selected { 18 } else { 16 })  // Plus grand si sélectionné
            )
            .on_press(Message::ServiceSelected(password.clone()))
            .width(Length::Fill)
            .style(if is_selected {
                iced::theme::Button::Primary  // Style différent si sélectionné
            } else {
                iced::theme::Button::Secondary
            });
    
            passwords_list = passwords_list.push(password_button);
        }
    
        // Section de statut
        let status_section = if let Some(message) = &self.status_message {
            Text::new(message).size(16)
        } else {
            Text::new("").size(16)
        };
    
        // Créer la section du mot de passe séparément
        let password_section = Column::new()
            .push(Text::new("Mot de passe").size(16))
            .push(
                TextInput::new(
                    "Mot de passe",
                    &self.password_value
                )
                .on_input(Message::InputChanged)
                .padding(10)
                .width(Length::Fill)
            );
        
        // Ajouter un texte d'avertissement si le mot de passe est trop court
        let password_section = if self.password_value.len() < 4 && !self.password_value.is_empty() {
            password_section.push(
                Text::new("Mot de passe trop court ")
                    .size(14)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb(0.9, 0.2, 0.2)))  // Rouge
            )
        } else {
            password_section
        };
    
        // Formulaire principal pour ajouter un nouveau mot de passe
        let main_form = Column::new()
            .spacing(20)
            .push(Text::new("Ajouter un nouveau mot de passe").size(20))
            .push(
                Column::new()
                    .push(Text::new("URL du service").size(16))
                    .push(
                        TextInput::new(
                            "URL du service (ex: www.exemple.com)",
                            &self.service_url_value
                        )
                        .on_input(Message::ServiceUrlChanged)
                        .padding(10)
                        .width(Length::Fill)
                    )
            )
            .push(password_section)  // Utilisez la section du mot de passe modifiée ici
            .push(
                Column::new()
                    .push(Text::new("Email").size(16))
                    .push(
                        TextInput::new(
                            "Email",
                            &self.email_value
                        )
                        .on_input(Message::EmailInputChanged)
                        .padding(10)
                        .width(Length::Fill)
                    )
            );
    
        // Boutons d'action
        let action_buttons = Row::new()
            .spacing(10)
            .push(
                Button::new(Text::new("Générer"))
                    .on_press(Message::GeneratePassword)
                    .padding(10)
            )
            .push(
                Button::new(Text::new("Copier"))
                    .on_press(Message::CopyToClipboard)
                    .padding(10)
            )
            .push(
                Button::new(Text::new("Sauvegarder"))
                    .on_press(Message::SavePassword)
                    .padding(10)
            );
    
        // Ajout des boutons au formulaire
        let main_form = main_form.push(action_buttons);
    
        // Section des mots de passe enregistrés
        let saved_passwords_section = Column::new()
            .spacing(20)
            .push(Text::new("Mots de passe enregistrés").size(20))
            .push(
                Button::new(Text::new("Actualiser"))
                    .on_press(Message::RefreshPasswords)
                    .padding(10)
            )
            .push(
                Container::new(
                    Scrollable::new(passwords_list)
                )
                .height(Length::Fill)
            );
    
        // Mise en page globale
        let content = Row::new()
            .spacing(40)
            .padding(20)
            .push(
                Column::new()
                    .width(Length::FillPortion(3))
                    .spacing(20)
                    .push(main_form)
                    .push(status_section)
            )
            .push(
                saved_passwords_section.width(Length::FillPortion(2))
            );
    
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
    
    // Vue détaillée d'un service avec ses informations
    fn view_service_detail(&self) -> Element<Message> {
        let service_name = self.selected_password.clone().unwrap_or_default();
        
        // En-tête avec le nom du service
        let header = Column::new()
            .spacing(10)
            .push(
                Row::new()
                    .spacing(20)
                    .push(Button::new(Text::new("< Retour")).on_press(Message::BackToMain))
                    .push(Text::new(format!("Détails du service: {}", service_name)).size(24))
            );
            
        // Message de statut
        let status_section = if let Some(message) = &self.status_message {
            Text::new(message).size(16)
        } else {
            Text::new("").size(16)
        };
        
        // Formulaire d'informations détaillées - on supprime le champ URL du service
        let detail_form = Column::new()
            .spacing(20)
            .push(
                Column::new()
                    .push(Text::new("Mot de passe").size(16))
                    .push(
                        TextInput::new(
                            "Mot de passe",
                            &self.password_value
                        )
                        .on_input(Message::InputChanged)
                        .padding(10)
                        .width(Length::Fill)
                    )
            )
            .push(
                Column::new()
                    .push(Text::new("Email").size(16))
                    .push(
                        TextInput::new(
                            "Email",
                            &self.email_value
                        )
                        .on_input(Message::EmailInputChanged)
                        .padding(10)
                        .width(Length::Fill)
                    )
            );
        
        // Boutons d'action
        let action_buttons = Row::new()
            .spacing(10)
            .push(
                Button::new(Text::new("Générer nouveau mot de passe"))
                    .on_press(Message::GeneratePassword)
                    .padding(10)
            )
            .push(
                Button::new(Text::new("Copier"))
                    .on_press(Message::CopyToClipboard)
                    .padding(10)
            )
            .push(
                Button::new(Text::new("Mettre à jour"))
                    .on_press(Message::UpdatePassword)
                    .padding(10)
            );
        
        // Mise en page complète
        let content = Column::new()
            .spacing(30)
            .padding(20)
            .push(header)
            .push(detail_form)
            .push(action_buttons)
            .push(status_section);
        
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}

#[allow(dead_code)]
pub fn main() -> iced::Result {
    PasswordManagerApp::run(Settings::default())
}