from flask import Flask, jsonify, request, make_response
from flask_cors import CORS
import random
import os
import json
import base64
from cryptography.fernet import Fernet

app = Flask(__name__)
CORS(app, resources={r"/*": {"origins": "*"}}, supports_credentials=True)

# Chemins vers les fichiers
PASSWORDS_FILE = os.path.join(os.path.dirname(__file__), 'passwords.json')
EMAILFILE =os.path.join(os.path.dirname(__file__), 'usermail.json')
KEY_FILE = 'key_file.key'
USER_MAIL = ""

#Fonctions utilitaires
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
    """Encrypte un mot de passe avec une clé et renvoie la version encodée en base64."""
    encrypted_password = key.encrypt(password.encode())
    return base64.b64encode(encrypted_password).decode('utf-8')  # Encodage en base64 pour stockage JSON

def decrypt_password(encrypted_password, key):
    """Décrypte un mot de passe avec une clé."""
    encrypted_password_bytes = base64.b64decode(encrypted_password)  # Décodage de base64
    return key.decrypt(encrypted_password_bytes).decode()

def extract_domain_name(url):
    """Extrait le nom de domaine d'une URL, juste avant '.fr' ou '.com'."""
    url = url.replace('http://', '').replace('https://', '').replace('www.', '')
    domain = url.split('/')[0]  # Prendre uniquement la partie avant le premier slash

    # On cherche à récupérer le nom avant '.fr', '.com', '.net', '.org'
    for tld in ['.fr', '.com', '.net', '.org']:
        if domain.endswith(tld):
            return domain.split(tld)[0]  # Retourne la partie avant le TLD
    return domain  # Si aucun TLD trouvé, retourne le domaine complet

def extract_service_name(url):
    """Extrait le nom du service (même sans point)."""
    url = url.replace('http://', '').replace('https://', '').replace('www.', '')
    domain = url.split('/')[0]  # Extraire la partie avant le premier slash
    parts = domain.split('.')
    
    if len(parts) >= 2:
        # Si on a un sous-domaine et un domaine
        return parts[0] + '.' + parts[1]
    else:
        # Si on n'a qu'un seul élément (pas de point)
        return parts[0]

def extract_service_url(url):
    """Extrait l'URL sans les paramètres après le domaine."""
    url = url.replace('http://', '').replace('https://', '')
    domain = url.split('/')[0]  # Extraire le domaine
    return f"https://www.{domain}"  # Inclure www

# --- Création des routes Flask ---

@app.route('/save-password', methods=['OPTIONS'])
def handle_options():
    response = make_response()
    response.headers.add("Access-Control-Allow-Origin", "*")
    response.headers.add("Access-Control-Allow-Headers", "Content-Type,Authorization")
    response.headers.add("Access-Control-Allow-Methods", "GET,PUT,POST,DELETE,OPTIONS")
    return response

@app.after_request
def add_cors_headers(response):
    response.headers['Access-Control-Allow-Origin'] = '*'
    response.headers['Access-Control-Allow-Headers'] = 'Content-Type,Authorization'
    response.headers['Access-Control-Allow-Methods'] = 'GET, POST, PUT, DELETE, OPTIONS'
    response.headers['Access-Control-Allow-Credentials'] = 'true'
    return response

@app.route('/')
def home():
    return jsonify({"message": "Bienvenue sur l'API Mushroom Password Manager!"})

@app.route('/generate-password', methods=['GET'])
def api_generate_password():
    length = request.args.get('length', default=25, type=int)
    password = generate_password(length)
    return jsonify({"password": password})

@app.route('/save-password', methods=['POST'])
def api_save_password():
    data = request.json
    print(f"Data reçue : {data}")  # Affiche les données reçues pour debug

    service_url = data.get('service')
    password = data.get('password')
    email = data.get('email', '')  # Valeur par défaut vide si non fournie

    if not service_url or not password:
        return jsonify({"error": "Les champs 'service' et 'password' sont requis."}), 400

    if not service_url or not password:
        return jsonify({"error": "Les champs 'service' et 'password' sont requis."}), 400

    ensure_json_file(PASSWORDS_FILE)
    try:
        key = load_key()
    except FileNotFoundError as e:
        return jsonify({"error": str(e)}), 500

    with open(PASSWORDS_FILE, 'r') as file:
        passwords = json.load(file)

    service_name = extract_service_name(service_url)  # Extraire le nom du service
    service_url_trimmed = extract_service_url(service_url)  # Extraire l'URL avec le sous-domaine
    encrypted_password = encrypt_password(password, key)  # Encrypté et encodé en base64

    passwords[service_url] = {
        'service_URL': service_url_trimmed,
        'service_name': service_name,
        'service_password': encrypted_password,
        'email': email or ''  # Garantit une chaîne vide si None
    }


    with open(PASSWORDS_FILE, 'w') as file:
        json.dump(passwords, file, indent=4)

    return jsonify({"message": f"Mot de passe enregistré pour le service '{service_name}'."})

@app.route('/list-passwords', methods=['GET'])
def api_list_passwords():
    try:
        ensure_json_file(PASSWORDS_FILE)
        key = load_key()  # Vérifier que load_key() ne lève pas d'exception
        with open(PASSWORDS_FILE, 'r') as file:
            passwords = json.load(file)

        for saved in passwords:
            encrypted_password = passwords[saved]['service_password']
            decrypted_password = decrypt_password(encrypted_password, key)
            passwords[saved]['service_password'] = decrypted_password
            
        return jsonify(passwords)  # Retourner les données brutes pour test

    except Exception as e:
        app.logger.error(f"ERREUR CRITIQUE: {str(e)}")  # Log dans la console
        return jsonify({"error": "Erreur de traitement"}), 500


@app.route('/api/v1/passwords', methods=['GET'])
def api_list_passwords_rust():
    ensure_json_file(PASSWORDS_FILE)
    try:
        key = load_key()
        with open(PASSWORDS_FILE, 'r') as file:
            encrypted_data = json.load(file)
            passwords = [f"{service}: (chiffré)" for service in encrypted_data.keys()]
            return jsonify(passwords)
    except Exception as e:
        return jsonify({"error": str(e)}), 500
@app.route('/get-password', methods=['POST'])
def api_get_password():
    print("=== ENDPOINT GET-PASSWORD APPELÉ ===")
    data = request.json
    print(f"Données reçues: {data}")
    service_url = data.get('service_URL')
    print(f"Service URL demandé: {service_url}")

    if not service_url:
        print("Erreur: service_URL manquant")
        return jsonify({"error": "Le champ 'service_URL' est requis."}), 400

    ensure_json_file(PASSWORDS_FILE)
    try:
        key = load_key()
    except FileNotFoundError as e:
        print(f"Erreur de clé: {e}")
        return jsonify({"error": str(e)}), 500

    with open(PASSWORDS_FILE, 'r') as file:
        passwords = json.load(file)
        print(f"Clés disponibles: {list(passwords.keys())}")

    if service_url in passwords:
        encrypted_password = passwords[service_url]['service_password']
        password = decrypt_password(encrypted_password, key)
        response_data = {
            "service_URL": service_url, 
            "service_name": passwords[service_url]['service_name'], 
            "service_password": password,
            "email": passwords[service_url].get('email', '')
        }
        print(f"Réponse: {response_data}")
        return jsonify(response_data)
    else:
        print(f"Aucun mot de passe trouvé pour: {service_url}")
        return jsonify({"error": f"Aucun mot de passe trouvé pour le service '{service_url}'."}), 404
        
@app.route('/registered', methods=['POST'])
def api_registered():
    data = request.json
    service_URL = data.get('service_URL')

    if not service_URL:
        return jsonify({"error": "Le champ 'service_URL' est requis."}), 400

    service_URL = extract_service_url(service_URL)

    ensure_json_file(PASSWORDS_FILE)
    try:
        key = load_key()
    except FileNotFoundError as e:
        return jsonify({"error": str(e)}), 500

    with open(PASSWORDS_FILE, 'r') as file:
        passwords = json.load(file)

    for service, details in passwords.items():
        if details['service_URL'] == service_URL:
            decrypted_password = decrypt_password(details['service_password'], key)
            return jsonify({
                "registered": True,
                "message": f"L'URL '{service_URL}' est déjà enregistrée.",
                "service_name": details['service_name'],
                "service_url": details['service_URL'],
                "email": details.get('email'),  # Ajout explicite de l'email
                "password": decrypted_password
            })

    return jsonify({
        "registered": False,
        "message": f"L'URL '{service_URL}' n'est pas enregistrée."
    })

@app.route('/getEmail', methods=["GET"])
def getEmail():
    ensure_json_file(EMAILFILE)
    try:
        with open(EMAILFILE, 'r') as file: 
            fichier = json.load(file)
        print("Voici le mail trouvé : ", fichier.get("email"))
        return jsonify({"email": fichier.get("email")})
    except FileNotFoundError as e:
        return jsonify({"email": ""})


@app.route('/changeMail', methods=["POST"])
def changeMail():
    data = request.json
    new_email = data.get('email')

    if not new_email:
        return jsonify({"error": "Le champ 'email' est requis."}), 400

    with open(EMAILFILE, 'w') as file:
        json.dump({"email": new_email}, file)

    return jsonify({"message": "L'email a été mis à jour avec succès."})

# --- Gestion des erreurs globales ---
@app.errorhandler(Exception)
def handle_exception(e):
    """Gestionnaire d'erreurs global pour toutes les exceptions non gérées."""
    app.logger.error(f"Exception non gérée : {e}")
    return jsonify({"error": "Une erreur interne est survenue."}), 500

# Lancement du serveur
if __name__ == '__main__':
    # Créer les fichiers de données s'ils n'existent pas
    if not os.path.exists(KEY_FILE):
        gen_key()
    ensure_json_file(PASSWORDS_FILE)
    ensure_json_file(EMAILFILE)
    app.run(debug=True)