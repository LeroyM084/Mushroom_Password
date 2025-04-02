use iced::{
    Application, Command, Element, Length, Settings, Subscription, Theme,
    widget::{Button, Column, Container, Row, Scrollable, Text, TextInput},
};
use iced::clipboard;

use crate::api::client::ApiClient;

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
}

pub struct PasswordManagerApp {
    // Valeurs des champs
    password_value: String,
    service_url_value: String,
    email_value: String,

    // État de l'application
    api_client: ApiClient,
    passwords: Vec<String>,
    selected_password: Option<String>,

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
                password_value: String::new(),
                service_url_value: String::new(),
                email_value: String::new(),
                api_client: ApiClient::new(),
                passwords: Vec::new(),
                selected_password: None,
                status_message: None,
            },
            Command::batch(vec![
                Command::perform(async { Message::GetEmail }, |_| Message::GetEmail)
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
                        Ok(_) => Message::StatusUpdate("Sauvegarde réussie".into()),
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
            //Ajouter ici 
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
        }
    }

    fn view(&self) -> Element<Message> {
        // Liste des mots de passe sauvegardés
        let mut passwords_list = Column::new().spacing(10);

        for password in &self.passwords {
            let password_button = Button::new(
                Text::new(password)
            )
            .on_press(Message::PasswordSelected(password.clone()))
            .width(Length::Fill);

            passwords_list = passwords_list.push(password_button);
        }

        // Section de statut
        let status_section = if let Some(message) = &self.status_message {
            Text::new(message).size(16)
        } else {
            Text::new("").size(16)
        };

        // Formulaire principal
        let main_form = Column::new()
            .spacing(20)
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
                    )
            )
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
                    )
            )
            .push(
                Row::new()
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
                    )
            );

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

        // Section email
        let email_section = Column::new()
            .spacing(20)
            .push(Text::new("Email").size(16))
            .push(
                TextInput::new(
                    "Email",
                    &self.email_value
                )
                .on_input(Message::EmailInputChanged)
                .padding(10)
            )
            .push(
                Button::new(Text::new("Sauvegarder Email"))
                    .on_press(Message::SaveEmail)
                    .padding(10)
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
                    .push(email_section)
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

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}


#[allow(dead_code)] // Ignorer l'avertissement pour cette fonction
pub fn main() -> iced::Result {
    PasswordManagerApp::run(Settings::default())
}
