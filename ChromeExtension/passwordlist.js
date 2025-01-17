// URL de l'API pour récupérer la liste des mots de passe
const API_URL = 'http://localhost:5000/list-passwords';

// Fonction pour récupérer les services et mots de passe depuis le JSON
async function recupererServicesEtMotsDePasse() {
    try {
        // Récupérer les données via l'API
        const response = await fetch(API_URL);
        if (!response.ok) {
            throw new Error(`Erreur lors de la récupération des données : ${response.statusText}`);
        }

        const motsDePasse = await response.json();

        // Vérifier si le JSON est vide
        if (!motsDePasse || Object.keys(motsDePasse).length === 0) {
            throw new Error('Aucun mot de passe enregistré.');
        }

        // Extraire les services et les mots de passe
        const services = Object.keys(motsDePasse);
        const passwords = Object.values(motsDePasse);

        // Vérifier que les deux listes ont la même longueur
        if (services.length !== passwords.length) {
            throw new Error('Incohérence dans les données : nombre de services et mots de passe différent.');
        }

        return { services, passwords };
    } catch (error) {
        console.error('Erreur:', error);
        return { services: [], passwords: [] };
    }
}

// Charger les données lorsque la page est prête
document.addEventListener('DOMContentLoaded', async () => {
    const resultDiv = document.getElementById('password-list');
    const copyIconPath = "/ChromeExtension/assets/imgs/copy_icon.png";
    const tickIconPath = "/ChromeExtension/assets/imgs/tick_icon.png";

    const result = await recupererServicesEtMotsDePasse();
    if (result) {
        const { services, passwords } = result;

        // Sélectionner l'élément de la liste
        const passwordList = document.getElementById('password-list');

        // Créer un élément de liste pour chaque service et mot de passe
        services.forEach((service, index) => {
            const listItem = document.createElement('li');
            listItem.classList.add('service-item');

            // Créer un bouton pour copier le mot de passe dans le presse-papier
            const button = document.createElement('button');
            button.classList.add('copy-button');
            button.innerHTML = `<img src="${copyIconPath}" alt="Copier" title="Copier le mot de passe">`;
            button.addEventListener('click', () => {
                const password = passwords[index];

                // Copier le mot de passe dans le presse-papier
                navigator.clipboard.writeText(password)
                    .then(() => {
                        // Changer l'icône après la copie
                        button.innerHTML = `<img src="${tickIconPath}" alt="Copié" title="Mot de passe copié">`;

                        // Revenir à l'icône de copie après 2 secondes
                        setTimeout(() => {
                            button.innerHTML = `<img src="${copyIconPath}" alt="Copier" title="Copier le mot de passe">`;
                        }, 2000);
                    })
                    .catch(err => {
                        console.error('Erreur lors de la copie : ', err);
                    });
            });

            // Créer un élément de texte pour afficher le service
            const serviceName = document.createElement('span');
            serviceName.classList.add('service-name');
            serviceName.textContent = service;

            // Ajouter le service et le bouton à l'élément de la liste
            listItem.appendChild(serviceName);
            listItem.appendChild(button);

            // Ajouter l'élément de la liste au DOM
            passwordList.appendChild(listItem);
        });
    } else {
        console.log('Aucune donnée disponible.');
    }
});
