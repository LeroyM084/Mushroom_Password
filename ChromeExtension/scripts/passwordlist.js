// Configuration
const API_URL = "http://localhost:5000/list-passwords";
const ICONS = {
  copy: "/ChromeExtension/assets/imgs/copy_icon.png",
  tick: "/ChromeExtension/assets/imgs/tick_icon.png",
};

// Fonction pour extraire le nom principal d'un service à partir d'une URL
function extractServiceName(url) {
  try {
    // Regex pour capturer uniquement la partie principale avant le .fr / .com / etc.
    const domainRegex =
      /https?:\/\/(?:www\.)?(?:[\w\-]+\.)?([\w\-]+)(?=\.\w{2,3}\/?)/;
    const match = url.match(domainRegex);
    if (match && match[1]) {
      // Met la première lettre en majuscule et le reste en minuscule
      return match[1].charAt(0).toUpperCase() + match[1].slice(1).toLowerCase();
    }
    return "Service inconnu"; // Si le nom ne peut pas être extrait
  } catch (error) {
    console.error("Erreur lors de l'extraction du nom du service :", error);
    return "Service inconnu";
  }
}

// Récupérer les services et mots de passe
async function fetchPasswords() {
  try {
    const response = await fetch(API_URL);
    if (!response.ok) throw new Error("Erreur de récupération");

    const passwords = await response.json();
    return passwords;
  } catch (error) {
    console.error("Erreur:", error);
    return {};
  }
}

// Copier un mot de passe
function copyPassword(password, button) {
  navigator.clipboard
    .writeText(password)
    .then(() => {
      button.innerHTML = `<img src="${ICONS.tick}" alt="Copié">`;
      setTimeout(() => {
        button.innerHTML = `<img src="${ICONS.copy}" alt="Copier">`;
      }, 2000);
    })
    .catch((err) => console.error("Erreur de copie:", err));
}

// Afficher les mots de passe
function displayPasswords(passwords) {
  const passwordList = document.getElementById("password-list");
  passwordList.innerHTML = ""; // Vider la liste

  Object.entries(passwords).forEach(([url, password]) => {
    const serviceName = extractServiceName(url); // Utiliser extractServiceName ici

    const listItem = document.createElement("li");
    listItem.classList.add("service-item");

    const serviceSpan = document.createElement("span");
    serviceSpan.textContent = serviceName; // Afficher le nom extrait
    serviceSpan.classList.add("service-name");

    const copyButton = document.createElement("button");
    copyButton.innerHTML = `<img src="${ICONS.copy}" alt="Copier">`;
    copyButton.addEventListener("click", () =>
      copyPassword(password, copyButton)
    );

    listItem.appendChild(serviceSpan);
    listItem.appendChild(copyButton);
    passwordList.appendChild(listItem);
  });
}

// Rechercher des mots de passe
function searchPasswords() {
  const searchInput = document
    .getElementById("search-input")
    .value.toLowerCase();
  const passwordItems = document.querySelectorAll(".service-item");

  passwordItems.forEach((item) => {
    const serviceName = item
      .querySelector(".service-name")
      .textContent.toLowerCase();
    item.style.display = serviceName.includes(searchInput) ? "flex" : "none";
  });
}

// Initialisation
document.addEventListener("DOMContentLoaded", async () => {
  const searchInput = document.getElementById("search-input");
  searchInput.addEventListener("input", searchPasswords);

  try {
    const passwords = await fetchPasswords();
    displayPasswords(passwords);
  } catch (error) {
    console.error("Erreur lors du chargement:", error);
  }
});
