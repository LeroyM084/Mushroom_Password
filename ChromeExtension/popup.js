function generatePassword() {
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
            document.getElementById("result").innerHTML = data.password;
        })
        .catch(error => {
            console.error("Erreur :", error);
            document.getElementById("result").innerHTML = "Erreur lors de la génération du mot de passe.";
        });
}


document.addEventListener('DOMContentLoaded', () => {
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

    // Appeler la fonction de génération de mot de passe au chargement
    generatePassword();
});
