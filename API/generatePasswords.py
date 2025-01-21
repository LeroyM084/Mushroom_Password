from flask_password_manager import *

gen_key()
service = "GITHUB"

save_password(service, generate_password())