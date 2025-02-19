document.addEventListener("DOMContentLoaded", () => {
	const form = document.querySelector("form");
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
