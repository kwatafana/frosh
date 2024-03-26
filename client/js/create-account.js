document.getElementById("CREATE_ACCOUNT_BTN").addEventListener("click", async () =>{
    let username = document.getElementById("USERNAME").value;
    let email = document.getElementById("EMAIL").value;
    let password = document.getElementById("PASSWORD").value;
    let repeatedPassword = document.getElementById("REAPEATED_PASSWORD").value;

    infoBox("😀 Creating account, please wait...", "primary");

    if (username == ""
	|| email == ""
	|| password == ""){
	infoBox("🙄 Fields cannot be empty", "error");
    } else{
	if(password !== repeatedPassword){
	    infoBox("😏 Passwords do not match", "error");
	} else {
	    let input = new KongAccountCreationInput(username, email, password);

	    try {
		await KongAccountsAPI.create_account(input);

		infoBox("😃 Account has been created", "success");
		document.location.href = '/portal/login.html';
	    } catch(e){
		infoBox(e, "error");
	    }

	}
    }
});
