let generatedPassword = 'TETS'; // Variable globale pour stocker le mot de passe

// Fonction asynchrone pour générer le mot de passe
async function generatePassword() {
    const length = 25; // Définir une longueur ou demander une entrée utilisateur
    try {
        const response = await fetch(`http://localhost:5000/generate-password?length=${length}`);

        if (!response.ok) {
            throw new Error("Erreur lors de l'appel à l'API");
        }

        const data = await response.json(); // Convertit la réponse en JSON
        console.log("Password généré : ", data); // Vérification dans la console
        document.getElementById("result").innerHTML = data.password;

        // Stocker le mot de passe dans la variable globale
        generatedPassword = data.password;

        return data;
    } catch (error) {
        console.error("Erreur :", error);
        document.getElementById("result").innerHTML = "Erreur lors de la génération du mot de passe.";
    }
}

document.addEventListener('DOMContentLoaded', async () => {
    const resultDiv = document.getElementById('result');
    const copyButton = document.getElementById('copy');
    const copyIconPath = "/ChromeExtension/assets/imgs/copy_icon.png";
    const tickIconPath = "/ChromeExtension/assets/imgs/tick_icon.png";

    // Initialiser l'icône du bouton
    copyButton.innerHTML = `<img src="${copyIconPath}" alt="Copier" title="Copier le mot de passe">`;

    copyButton.addEventListener('click', () => {
        const password = resultDiv.textContent;

        // Copier le mot de passe dans le presse-papier
        navigator.clipboard.writeText(password)
            .then(() => {
                // Changer l'icône après la copie
                copyButton.innerHTML = `<img src="${tickIconPath}" alt="Copié" title="Mot de passe copié">`;

                // Revenir à l'icône de copie après 2 secondes
                setTimeout(() => {
                    copyButton.innerHTML = `<img src="${copyIconPath}" alt="Copier" title="Copier le mot de passe">`;
                }, 2000);
            })
            .catch(err => {
                console.error('Erreur lors de la copie : ', err);
            });
    });

    // Attendre que le mot de passe soit généré avant d'appeler saveURLAndPassword
    await generatePassword();
    saveURLAndPassword();
});

// Utilisation de la variable globale dans une autre fonction
async function saveURLAndPassword() {
    chrome.tabs.query({ active: true, currentWindow: true }, function(tabs) {
        let currentURL = tabs[0].url; // Permet de sauvegarder l'URL actuelle.
        console.log("URL actuelle : ", currentURL);
    });
    let currentPassword = generatePassword();


}
