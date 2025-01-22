let mot_de_passe = "";

// Fonction pour détecter les champs de type "password"
function detectPasswordFields() {
  console.log("=== Début de detectPasswordFields ===");
  try {
    const passwordFields = document.querySelectorAll('input[type="password"]');
    console.log(`Nombre de champs password trouvés: ${passwordFields.length}`);

    if (passwordFields.length > 0) {
      passwordFields.forEach((field, index) => {
        console.log(`Champ ${index + 1} détecté:`, {
          id: field.id,
          name: field.name,
          class: field.className,
        });
      });
    } else {
      console.log("Aucun champ password détecté.");
    }
  } catch (error) {
    console.error("Erreur dans detectPasswordFields:", error);
  }
}

// Fonction pour remplir les champs de type "password"
async function fillPasswordFields() {
  console.log("=== Début de fillPasswordFields ===");
  try {
    const passwordFields = document.querySelectorAll('input[type="password"]');
    console.log(
      `Nombre de champs password détectés à remplir: ${passwordFields.length}`
    );

    if (passwordFields.length === 0) {
      console.log("Aucun champ à remplir, fonction terminée.");
      return;
    }

    const url = window.location.href;
    console.log(`URL actuelle détectée : ${url}`);

    // Vérifie si un mot de passe est déjà enregistré pour le service
    const registeredResponse = await fetch("http://localhost:5000/registered", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ service_URL: url }),
    });

    if (!registeredResponse.ok) {
      throw new Error(
        `Erreur lors de la vérification du mot de passe enregistré : ${registeredResponse.statusText}`
      );
    }

    const registeredData = await registeredResponse.json();
    console.log("Réponse de l'API de vérification :", registeredData);

    if (registeredData.registered) {
      // Si un mot de passe est déjà enregistré
      mot_de_passe = registeredData.password;
      console.log(
        `Mot de passe déjà enregistré pour '${registeredData.service_name}': ${mot_de_passe}`
      );
    } else {
      // Sinon, générer un nouveau mot de passe
      const length = 25;
      console.log(
        `Tentative de génération du mot de passe avec une longueur de ${length} caractères.`
      );

      const response = await fetch(
        `http://localhost:5000/generate-password?length=${length}`
      );

      if (!response.ok) {
        throw new Error(
          `Erreur lors de l'appel à l'API de génération : ${response.statusText}`
        );
      }

      const data = await response.json();
      mot_de_passe = data.password; // Stocker le mot de passe généré
      console.log("Mot de passe généré reçu depuis l'API :", mot_de_passe);

      // Sauvegarder le mot de passe généré
      console.log("Lancement de la sauvegarde du mot de passe.");
      await saveServiceAndPasswordInJSON(mot_de_passe);
    }

    // Remplir les champs password
    passwordFields.forEach((field, index) => {
      field.value = mot_de_passe;
      console.log(`Champ ${index + 1} rempli avec le mot de passe.`);
    });
  } catch (error) {
    console.error("Erreur dans fillPasswordFields:", error);
  }
}

// Fonction pour sauvegarder le service et le mot de passe dans un JSON via l'API
async function saveServiceAndPasswordInJSON(passwordToSave) {
  console.log("=== Début de saveServiceAndPasswordInJSON ===");

  try {
    const url = window.location.href;
    console.log(`URL actuelle détectée : ${url}`);

    const data = { service: url, password: passwordToSave }; // Envoi des données avec le mot de passe
    console.log("Données à sauvegarder :", data);

    const response = await fetch("http://localhost:5000/save-password", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      throw new Error(`Erreur lors de la sauvegarde : ${response.statusText}`);
    }

    console.log("Données sauvegardées avec succès :", data);
  } catch (error) {
    console.error("Erreur dans saveServiceAndPasswordInJSON:", error);
  }
}

// Fonction principale d'exécution
function main() {
  console.log("=== Début de l'exécution principale ===");
  detectPasswordFields();
  fillPasswordFields();
  console.log("=== Exécution principale terminée ===");
}

// Exécution lorsque le DOM est chargé
document.addEventListener("DOMContentLoaded", () => {
  console.log("=== DOM Content Loaded ===");
  main();
});

// Observation des changements dans le DOM
const observer = new MutationObserver(() => {
  console.log("=== Mutation DOM détectée ===");
  detectPasswordFields();
  fillPasswordFields();
});

// Démarrer l'observation des changements dans le DOM
observer.observe(document.documentElement, {
  childList: true,
  subtree: true,
});

console.log("=== Content Script : Configuration terminée ===");
