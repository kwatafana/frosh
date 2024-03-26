document.getElementById("LOGIN_BTN").addEventListener("click", async () =>{
    let username = document.getElementById("USERNAME").value;
    let password = document.getElementById("PASSWORD").value;

    infoBox("😀 Unlocking account, please wait...", "primary");

    if (username == ""
	|| password == ""){
	infoBox("🙄 Fields cannot be empty", "error");
    } else{
	let input = new KongAccountAuthInput(username, password);

	try {
	    await KongLoginAPI.authenticate(input);

	    infoBox("😃 Account has been unlocked", "success");
	    document.location.href = '/portal/dashboard.html';
	} catch(e){
	    infoBox(e, "error");
	}
    }
});
