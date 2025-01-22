from flask import Flask, jsonify, request
import random
import os
import json
from cryptography.fernet import Fernet
from flask_cors import CORS  # Import Flask-CORS

app = Flask(__name__)

# Activer CORS pour toutes les routes
CORS(app)

# Chemins vers les fichiers
PASSWORDS_FILE = '.\API\passwords.json'
KEY_FILE = 'key_file.key'


# --- Fonctions utilitaires ---
def ensure_json_file(file_path):
    """Assure que le fichier JSON existe et contient un dictionnaire vide."""
    if not os.path.exists(file_path) or os.path.getsize(file_path) == 0:
        with open(file_path, 'w') as file:
            json.dump({}, file)  # Initialise un dictionnaire JSON vide


def generate_password(length=25):
    """Génère un mot de passe aléatoire."""
    caracteres = ["/", "*", "-", "+", "=", ":", ";", ".", ",", "?", "!", "'", "(", ")",
                  "[", "]", "{", "}", "|", "&", "%", "$", "#", "@", "^", "~", "_"]
    alphabet_min = [chr(i) for i in range(97, 123)]  # Lettres minuscules
    alphabet_maj = [chr(i) for i in range(65, 91)]  # Lettres majuscules
    all_characters = caracteres + alphabet_min + alphabet_maj
    return ''.join(random.choice(all_characters) for _ in range(length))


def load_key():
    """Charge la clé de chiffrement depuis le fichier."""
    if not os.path.exists(KEY_FILE) or os.path.getsize(KEY_FILE) == 0:
        raise FileNotFoundError("Le fichier contenant la clé est introuvable ou vide. Générez une clé d'abord.")
    with open(KEY_FILE, "rb") as key_file:
        return Fernet(key_file.read())


def gen_key():
    """Génère une clé de chiffrement unique."""
    if not os.path.exists(KEY_FILE) or os.path.getsize(KEY_FILE) == 0:
        key = Fernet.generate_key()
        with open(KEY_FILE, "wb") as key_file:
            key_file.write(key)


def encrypt_password(password, key):
    """Encrypte un mot de passe avec une clé."""
    return key.encrypt(password.encode())


def decrypt_password(encrypted_password, key):
    """Décrypte un mot de passe avec une clé."""
    return key.decrypt(encrypted_password).decode()

# Fonction pour avoir le service à partir de l'URL.
def extract_domain_name(url):
    # Supprimer http:// ou https://
    url = url.replace('http://', '').replace('https://', '')
    
    # Supprimer www.
    url = url.replace('www.', '')
    
    # Couper à partir du premier point
    domain = url.split('.')[0]
    
    return domain


# --- Création des routes Flask ---
@app.route('/')
def home():
    return jsonify({"message": "Bienvenue sur l'API Mushroom Password Manager!"})


@app.route('/generate-password', methods=['GET'])
def api_generate_password():
    """Endpoint pour générer un mot de passe."""
    length = request.args.get('length', default=25, type=int)
    password = generate_password(length)
    return jsonify({"password": password})


@app.route('/save-password', methods=['POST'])
def api_save_password():
    """Endpoint pour enregistrer un mot de passe."""
    print("requete ok")
    print("donnee", request.json)
    data = request.json
    service = data.get('service')
    password = data.get('password')

    if not service or not password:
        return jsonify({"error": "Les champs 'service' et 'password' sont requis."}), 400

    ensure_json_file(PASSWORDS_FILE)
    key = load_key()

    # Charger les mots de passe existants
    with open(PASSWORDS_FILE, 'r') as file:
        passwords = json.load(file)

    # Modifier le nom de service pour le trim
    service = extract_domain_name(service)

    # Enregistrer le mot de passe chiffré
    encrypted_password = encrypt_password(password, key)
    passwords[service] = encrypted_password.decode()  # Convertir les bytes en chaîne

    with open(PASSWORDS_FILE, 'w') as file:
        json.dump(passwords, file)

    return jsonify({"message": f"Mot de passe enregistré pour le service '{service}'."})


@app.route('/list-passwords', methods=['GET'])
def api_list_passwords():
    """Endpoint pour lister tous les services et mots de passe enregistrés, décryptés."""
    ensure_json_file(PASSWORDS_FILE)

    # Lire la clé de décryptage depuis key.txt
    decryption_key = load_key()

    # Ouvrir et lire le fichier des mots de passe
    with open(PASSWORDS_FILE, 'r') as file:
        passwords = json.load(file)

    # Décrypter chaque mot de passe
    for service, encrypted_password in passwords.items():
        encrypted_password_bytes = encrypted_password.encode()  # Convertir en bytes
        decrypted_password = decrypt_password(encrypted_password_bytes, decryption_key)
        passwords[service] = decrypted_password

    return jsonify(passwords)

@app.route('/get-password', methods=['POST'])
def api_get_password():
    """Endpoint pour récupérer un mot de passe pour un service donné."""
    data = request.json
    service = data.get('service')

    if not service:
        return jsonify({"error": "Le champ 'service' est requis."}), 400

    ensure_json_file(PASSWORDS_FILE)
    key = load_key()

    with open(PASSWORDS_FILE, 'r') as file:
        passwords = json.load(file)

    if service in passwords:
        encrypted_password = passwords[service].encode()  # Convertir en bytes
        password = decrypt_password(encrypted_password, key)
        return jsonify({"service": service, "password": password})
    else:
        return jsonify({"error": f"Aucun mot de passe trouvé pour le service '{service}'."}), 404


# Lancement du serveur
if __name__ == '__main__':
    if not os.path.exists(KEY_FILE):
        gen_key()  # Génération de la clé si elle n'existe pas
    app.run(debug=True)










def save_password(service, password):
    """Endpoint pour enregistrer un mot de passe."""
    # data = request.json
    # service = data.get('service')
    # password = data.get('password')

    if not service or not password:
        return "Les champs 'service' et 'password' sont requis."

    ensure_json_file(PASSWORDS_FILE)
    key = load_key()

    # Charger les mots de passe existants
    with open(PASSWORDS_FILE, 'r') as file:
        passwords = json.load(file)

    # Enregistrer le mot de passe chiffré
    encrypted_password = encrypt_password(password, key)
    passwords[service] = encrypted_password.decode()  # Convertir les bytes en chaîne

    with open(PASSWORDS_FILE, 'w') as file:
        json.dump(passwords, file)

    return "OK mot de passe enregsitré correment."