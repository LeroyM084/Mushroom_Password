// Attendre que le DOM soit chargé
document.addEventListener('DOMContentLoaded', () => {
    console.log("Analyse démarrée ...");
    detectPasswordFields();
});

// Fonction qui recherche les champs de type 'password'
function detectPasswordFields() {
    const passwordFields = document.querySelectorAll('input[type="password"]');
    if (passwordFields.length > 0) {
        console.log(`Champs mots de passe détectés : ${passwordFields.length}`);
        // Tu peux ici envoyer des données à ton fond ou effectuer d'autres actions
    } else {
        console.log("Aucun champ de mot de passe détecté");
    }
}