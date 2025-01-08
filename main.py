import random
import json
from cryptography.fernet import Fernet
import os


def generate_password(length=25):
    caracteres = ["/", "*", "-", "+", "=", ":", ";", ".", ",", "?", "!", "'", "(", ")",
                  "[", "]", "{", "}", "|", "&", "%", "$", "#", "@", "^", "~", "_"]
    alphabet_min = [chr(i) for i in range(97, 123)]  # Lettres minuscules
    alphabet_maj = [chr(i) for i in range(65, 91)]  # Lettres majuscules
    all_characters = caracteres + alphabet_min + alphabet_maj
    return ''.join(random.choice(all_characters) for _ in range(length))


def ensure_json_file(file_path):
    """Assure que le fichier JSON existe et contient un dictionnaire vide."""
    if not os.path.exists(file_path) or os.path.getsize(file_path) == 0:
        with open(file_path, 'w') as file:
            json.dump({}, file)  # Crée un dictionnaire JSON vide


def save_password(service, password):
    ensure_json_file('API/passwords.json')  # Vérifie et initialise le fichier JSON si nécessaire
    key = load_key()
    encrypted_password = encrypt_password(password, key)

    with open('API/passwords.json', 'r') as file:
        passwords = json.load(file)

    if service in passwords:
        print(f"Le service '{service}' existe déjà. Le mot de passe sera mis à jour.")

    passwords[service] = encrypted_password.decode()  # Convertir bytes en string pour JSON

    with open('API/passwords.json', 'w') as file:
        json.dump(passwords, file)
    print(f"Mot de passe enregistré pour le service '{service}'.")


def get_password(service):
    ensure_json_file('API/passwords.json')  # Vérifie et initialise le fichier JSON si nécessaire
    key = load_key()

    with open('API/passwords.json', 'r') as file:
        passwords = json.load(file)

    if service in passwords:
        encrypted_password = passwords[service].encode()  # Convertir string en bytes
        return decrypt_password(encrypted_password, key)
    else:
        print(f"Aucun mot de passe trouvé pour le service '{service}'.")
        return None


def list_services():
    ensure_json_file('API/passwords.json')  # Vérifie et initialise le fichier JSON si nécessaire
    with open('API/passwords.json', 'r') as file:
        passwords = json.load(file)
        print("Services enregistrés :")
        for service in passwords.keys():
            print(f"- {service}")

def gen_key():
    # Générer et enregistrer une clé de chiffrement unique
    if not os.path.exists("API/key_file.key") or os.path.getsize("API/key_file.key") == 0:
        key = Fernet.generate_key()
        with open("API/key_file.key", "wb") as key_file:
            key_file.write(key)
        print("Clé de chiffrement générée et sauvegardée.")
    else:
        print("Clé de chiffrement existante détectée, aucune nouvelle clé générée.")


def load_key():
    # Charger la clé à partir du fichier
    if not os.path.exists("API/key_file.key") or os.path.getsize("API/key_file.key") == 0:
        raise FileNotFoundError("Le fichier de clé est introuvable ou vide. Générez une clé avec 'gen_key()'.")

    with open("API/key_file.key", "rb") as key_file:
        try:
            return Fernet(key_file.read())
        except ValueError:
            raise ValueError("Le fichier de clé contient des données invalides. Supprimez-le et générez une nouvelle clé.")


def encrypt_password(passwd, key):
    # Encrypter un mot de passe avec une clé
    return key.encrypt(passwd.encode())


def decrypt_password(passwd, key):
    # Décrypter un mot de passe avec une clé
    return key.decrypt(passwd).decode()


# Interface utilisateur simple
if __name__ == "__main__":
    ensure_json_file("API/passwords.json")
    gen_key()  # À exécuter une seule fois pour créer la clé

    while True:
        print("\n=== Gestionnaire de mots de passe ===")
        print("1. Générer et enregistrer un mot de passe")
        print("2. Récupérer un mot de passe")
        print("3. Lister tous les services enregistrés")
        print("4. Quitter")

        choix = input("Votre choix : ")

        if choix == "1":
            service = input("Entrez le nom du service : ")
            length = input("Longueur du mot de passe (par défaut : 25) : ")
            length = int(length) if length.isdigit() else 25
            password = generate_password(length)
            save_password(service, password)

        elif choix == "2":
            service = input("Entrez le nom du service : ")
            retrieved_password = get_password(service)
            if retrieved_password:
                print(f"Mot de passe pour '{service}' : {retrieved_password}")

        elif choix == "3":
            list_services()

        elif choix == "4":
            print("Au revoir !")
            break

        else:
            print("Choix invalide, veuillez réessayer.")
