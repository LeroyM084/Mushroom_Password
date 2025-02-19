let motDePasse = "";

// Fonction pour détecter les champs de type "password"
function detectPasswordFields() {
  try {
    const passwordFields = document.querySelectorAll(
      'input[type="password"]:not([data-filled="true"])'
    );

    if (passwordFields.length > 0) {
      return passwordFields;
    } else {
      console.log("Aucun champ password détecté.");
      return [];
    }
  } catch (error) {
    console.error("Erreur dans detectPasswordFields:", error);
    return [];
  }
}

// Fonction pour détecter les champs de type "email"
function detectEmailFields() {
  try {
    const emailFields = document.querySelectorAll(
      'input:not([type="password"]):not([data-filled="true"])'
    );

    if (emailFields.length > 0) {
      return emailFields;
    } else {
      console.log("🍄 => Aucun champ email détecté.");
      return [];
    }
  } catch (error) {
    console.error("Erreur dans detectEmailFields:", error);
    return [];
  }
}

// Fonction pour remplir les champs "email"
function fillEmailFields(mail) {
  try {
    const emailFields = detectEmailFields();
    emailFields.forEach((field) => {
      field.value = mail;
      field.setAttribute("data-filled", "true");
    });
    return true;
  } catch (error) {
    console.error("Erreur dans fillEmailFields:", error);
    return false;
  }
}

// Fonction pour vérifier si un mot de passe est déjà enregistré
async function alreadyRegistered(serviceURL = window.location.href) {
  try {
    const response = await fetch("http://localhost:5000/registered", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ service_URL: serviceURL }),
    });

    if (!response.ok) {
      throw new Error(
        `Erreur lors de la vérification : ${response.statusText}`
      );
    }

    const registeredData = await response.json();
    if (registeredData.registered) {
      return { password: registeredData.password, email: registeredData.email };
    }
    return { password: false, email: false };
  } catch (error) {
    console.error("Erreur dans alreadyRegistered:", error);
    return { password: false, email: false };
  }
}

// Fonction pour générer un mot de passe
async function generatePassword() {
  const API_URL = "http://localhost:5000/generate-password";
  const length = 25;

  try {
    const response = await fetch(`${API_URL}?length=${length}`);
    if (!response.ok) {
      throw new Error(`Erreur lors de la génération : ${response.statusText}`);
    }
    const data = await response.json();
    console.log("🍄 => Mot de passe généré :", data.password);
    return data.password;
  } catch (error) {
    console.error("Erreur dans generatePassword:", error);
    return null;
  }
}

// Fonction pour remplir les champs "password"
function fillPasswordFields(password) {
  try {
    const passwordFields = detectPasswordFields();
    passwordFields.forEach((field) => {
      field.value = password;
      field.setAttribute("data-filled", "true");
    });
    return true;
  } catch (error) {
    console.error("Erreur dans fillPasswordFields:", error);
    return false;
  }
}

// Fonction pour sauvegarder le mot de passe
async function savePassword(serviceURL, password) {
  try {
    const API_URL = "http://localhost:5000/save-password";
    const data = { service: serviceURL, password };

    const response = await fetch(API_URL, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      throw new Error(`Erreur lors de la sauvegarde : ${response.statusText}`);
    }

    console.log(
      "Mot de passe sauvegardé avec succès pour le service :",
      serviceURL
    );
  } catch (error) {
    console.error("Erreur dans savePassword:", error);
  }
}

function getMailAPI(){
  const response = fetch('localhost:5000/getEmail', methods=["GET"])
  if (!response.ok){
    console.error("Erreur lors de la récupération de l'email", error)
  }
  const userMail = response 
}

// Fonction principale
async function main() {
  const passwordFields = detectPasswordFields();
  if (passwordFields.length === 0) {
    console.log("🍄 => Aucun champ password détecté, fin de l'exécution.");
    return;
  }

  const serviceURL = window.location.href;
  const data = await alreadyRegistered(serviceURL);
  let password = data.password;
  let email = data.email;

  if (!password) {
    password = await generatePassword();
    console.log("🍄 => On génère un nouveau mot de passe car non-trouvé.");
  } else {
    // console.log("🍄 -> On utilise le mot de passe déjà enregistré.");
  }

  if (password) {
    fillPasswordFields(password);
    fillEmailFields(email);
  }
}

// Attendre 3 secondes après le chargement de la page avant d'exécuter main()
window.addEventListener("load", () => {
  setTimeout(main, 1500);
});
