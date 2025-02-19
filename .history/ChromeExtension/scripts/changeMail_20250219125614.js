document.addEventListener("DOMContentLoaded", async () => {
    const container = document.querySelector(".container");
    const form = document.querySelector("#emailForm");
    form.style.display = 'none'; // Cache le formulaire initialement

    // Création de la div pour afficher l'email actuel
    const currentEmailDiv = document.createElement("div");
    currentEmailDiv.id = "currentEmail";
    currentEmailDiv.className = "email-display";
    container.insertBefore(currentEmailDiv, form);

    try {
        // Récupération de l'email actuel
        const response = await fetch("http://localhost:5000/getEmail");
        const data = await response.json();
        
        // Affichage de l'email et du bouton
        currentEmailDiv.innerHTML = `
            <p>Adresse e-mail actuelle : <strong>${data.email}</strong></p>
            <button id="changeButton" type="button">Modifier</button>
        `;

        // Gestionnaire pour le bouton "Modifier"
        document.getElementById("changeButton").addEventListener("click", () => {
            currentEmailDiv.style.display = 'none';
            form.style.display = 'flex';
        });

    } catch (error) {
        console.error("Erreur lors de la récupération de l'email:", error);
        currentEmailDiv.innerHTML = `
            <p class="error">Erreur lors de la récupération de l'email</p>
            <button id="changeButton" type="button">Modifier</button>
        `;
    }

    // Reste du code existant pour la gestion du formulaire
    const emailInput = document.querySelector('input[type="email"]');
    const messageDiv = document.createElement("div");
    messageDiv.id = "message";
    form.appendChild(messageDiv);

    form.addEventListener("submit", async (e) => {
        e.preventDefault();
        const email = emailInput.value;

        try {
            const response = await fetch("http://localhost:5000/changeMail", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ email: email }),
            });

            const data = await response.json();

            if (!response.ok) {
                throw new Error(
                    data.error || "Erreur lors du changement d'email"
                );
            }

            // Afficher le message de succès
            messageDiv.innerHTML = `
                <div class="success">
                    ${data.message}<br>
                    <a href="index.html">Retour à l'accueil</a>
                </div>
            `;
        } catch (error) {
            console.error("Erreur:", error);
            messageDiv.innerHTML = `
                <div class="error">
                    Une erreur est survenue: ${error.message}<br>
                    <a href="index.html">Retour à l'accueil</a>
                </div>
            `;
        }
    });
});
