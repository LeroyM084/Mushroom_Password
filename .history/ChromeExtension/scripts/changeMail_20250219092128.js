const textFieldValue = document.querySelector('input[type="email"]').value;
const submitButton = document.getElementById("submitButton");

submitButton.addEventListener(onclick, saveMailAPI())

async function saveMailAPI(){
    try{
        const response = await fetch("http://localhost:5000/changeMail", 
            method = "POST"
        )
    }
}