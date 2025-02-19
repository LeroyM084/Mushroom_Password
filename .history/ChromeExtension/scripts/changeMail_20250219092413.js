const textFieldValue = document.querySelector('input[type="email"]').value;
const submitButton = document.getElementById("submitButton");

submitButton.addEventListener(onclick, saveMailAPI())

async function saveMailAPI(){
    try{
        const response = await fetch("http://localhost:5000/changeMail", {
            method = "POST",
            headers : {
                'Content-Type' : "application/json",
            },
            body : JSON.stringify({email : textFieldValue })
    }),   
    if (!response.ok) {
        console.error(response.status)
    }
    const data = await response.json, 
    }
}