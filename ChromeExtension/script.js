   // script.js

   document.getElementById("listPasswords").addEventListener("click", () => {
       fetch("http://localhost:5000/list-passwords")
           .then(response => response.json())
           .then(data => {
               document.getElementById("result").innerHTML = JSON.stringify(data.services);
           })
           .catch(error => {
               console.error("Erreur:", error);
           });
   });

   document.getElementById("generatePassword").addEventListener("click", () => {
    const length = 25; // Définir une longueur ou demander une entrée utilisateur
    fetch(`http://localhost:5000/generate-password?length=${length}`)
        .then(response => {
            if (!response.ok) {
                throw new Error("Erreur lors de l'appel à l'API");
            }
            return response.json(); // Convertit la réponse en JSON
        })
        .then(data => {
            console.log("Password généré : ", data); // Vérification dans la console
            document.getElementById("result").innerHTML = `Mot de passe généré : ${data.password}`;
        })
        .catch(error => {
            console.error("Erreur :", error);
            document.getElementById("result").innerHTML = "Erreur lors de la génération du mot de passe.";
        });
});


  /**
 * Fonction pour obtenir un mot de passe à partir d'un service
 */
function getPassword() {
    // Demander le service à l'utilisateur
    const service = prompt("Entrez le nom du service:");

    // Vérifier si le nom du service est valide
    if (!service || service.trim() === "") {
        alert("Le nom du service est obligatoire !");
        return;
    }

    // Effectuer la requête pour récupérer le mot de passe
    fetch("http://localhost:5000/get-password", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ service: service.trim() }) // Assurez-vous d'envoyer un JSON propre
    })
        .then(response => {
            if (!response.ok) {
                // Si une erreur survient côté API, lire l'erreur renvoyée
                return response.json().then(err => {
                    throw new Error(err.error || "Erreur inconnue lors de la récupération du mot de passe.");
                });
            }
            return response.json(); // Convertir en JSON si la réponse est OK
        })
        .then(data => {
            // Afficher le résultat ou alerter en cas de succès
            document.getElementById("result").innerHTML =
                `<p>Mot de passe pour <b>${data.service}</b> : <code>${data.password}</code></p>`;
        })
        .catch(error => {
            // Gérer les erreurs et les afficher proprement
            console.error("Erreur lors de la récupération :", error);
            document.getElementById("result").innerHTML =
                `<p style="color: red;">Erreur : ${error.message}</p>`;
        });
}

// Associer la fonction à un événement (par exemple, un bouton)
document.getElementById("getPassword").addEventListener("click", getPassword);


   // Fonction pour afficher tous les mots de passe
function displayPasswords() {
    console.log("Debug : displayPasswords()")
    // Conteneur où afficher les mots de passe
    const passwordListContainer = document.getElementById("passwordListContainer");

    // Récupérer la liste des services depuis l'API
    fetch("http://localhost:5000/list-passwords")
        .then(response => {
            if (!response.ok) {
                throw new Error("Erreur lors de la récupération des services");
            }
            return response.json();
        })
        .then(data => {
            // Effacer tout contenu précédent dans le conteneur
            passwordListContainer.innerHTML = "";

            // Récupérer chaque mot de passe en fonction du service
            const services = data.services;
            if (services.length === 0) {
                passwordListContainer.innerHTML = "<p>Aucun mot de passe enregistré pour l'instant.</p>";
                return;
            }

            // Pour chaque service, récupérer et afficher le mot de passe
            services.forEach(service => {
                fetch("http://localhost:5000/get-password", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify({ service })
                })
                .then(response => {
                    if (!response.ok) {
                        throw new Error(`Erreur pour le service : ${service}`);
                    }
                    return response.json();
                })
                .then(data => {
                    // Ajouter chaque service et mot de passe au conteneur
                    const passwordItem = document.createElement("p");
                    passwordItem.textContent = `"${data.service}" : "${data.password}"`;
                    passwordListContainer.appendChild(passwordItem);
                })
                .catch(error => {
                    console.error("Erreur:", error);
                });
            });
        })
        .catch(error => {
            console.error("Erreur lors de la récupération des services :", error);
            passwordListContainer.innerHTML = "<p>Erreur lors de la récupération de la liste des mots de passe.</p>";
        });
}

// Charger les mots de passe lorsqu'on charge la page
document.getElementById("displayPasswords").addEventListener("click", displayPasswords);