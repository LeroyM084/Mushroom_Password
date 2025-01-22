// background.js

import { generatePassword } from './scripts/generatePassword.js';

// Écouter les messages de la part de contentAnalyzer.js
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.action === 'generatePassword') {
    generatePassword().then(password => {
      sendResponse({ password });
    }).catch(error => {
      sendResponse({ error: 'Erreur lors de la génération du mot de passe' });
    });
  }
  return true; // pour indiquer que la réponse est asynchrone
});
