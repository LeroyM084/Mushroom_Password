let generatedPassword = ''; // Variable globale pour stocker le mot de passe généré

// Fonction asynchrone pour générer le mot de passe
async function generatePassword() {
const length = 25; // Longueur du mot de passe
try {
const response = await fetch(`http://localhost:5000/generate-password?length=${length}`);
if (!response.ok) {
    throw new Error("Erreur lors de l'appel à l'API");
}
const data = await response.json();
console.log("Mot de passe généré : ", data);
document.getElementById("result").textContent = data.password;
generatedPassword = data.password; // Stockage dans la variable globale
return data.password;
} catch (error) {
console.error("Erreur :", error);
document.getElementById("result").textContent = "Erreur lors de la génération du mot de passe.";
return null;
}
}

// Fonction pour sauvegarder l'URL et le mot de passe
async function saveURLAndPassword() {
return new Promise((resolve) => {
chrome.tabs.query({ active: true, currentWindow: true }, function(tabs) {
    resolve({
        service: tabs[0].url,
        password: generatedPassword
    });
});
});
}

// Fonction pour envoyer les données au serveur
async function saveServiceAndPasswordInJSON(donnee) {
console.log("Début de la sauvegarde");
try {
console.log("Données à envoyer :", JSON.stringify(donnee));
const response = await fetch("http://localhost:5000/save-password", {
    method: 'POST',
    headers: {
        "Content-Type": "application/json"
    },
    body: JSON.stringify(donnee)
});
console.log("Requête envoyée avec succès");
changeSaveButton()
return await response.json();
} catch (error) {
console.error("Erreur lors de la sauvegarde:", error);
}
}

// Initialisation et gestion des événements
document.addEventListener('DOMContentLoaded', async () => {
const resultDiv = document.getElementById('result');
const copyButton = document.getElementById('copy');
const saveButton = document.getElementById('save');
const copyIconPath = "/ChromeExtension/assets/imgs/copy_icon.png";
const tickIconPath = "/ChromeExtension/assets/imgs/tick_icon.png";

// Gestionnaire d'événement pour le bouton de sauvegarde
saveButton.addEventListener('click', async () => {
if (!generatedPassword) {
    await generatePassword();
}
const data = await saveURLAndPassword();
const result = await saveServiceAndPasswordInJSON(data);
console.log("Résultat de la sauvegarde:", result);
});
});


// Gérer l'état du bouton de sauvegarde 
function changeSaveButton() {
	const saveButton = document.getElementById('save');
	saveButton.textContent = "Sauvegardé ✓"; // Changer le texte
	saveButton.disabled = true; // Désactiver le bouton après sauvegarde
	saveButton.style.backgroundColor = "#4CAF50"; // Optionnel : changer la couleur
}
