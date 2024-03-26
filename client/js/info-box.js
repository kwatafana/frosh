function infoBox(info, state){
    let box = document.getElementById("INFO_BOX");
    clearState(box);
    box.style.display = "block";
    switch(state){
    case "primary":
	box.classList.add("toast-primary");
	break;
    case "success":
	box.classList.add("toast-success");
	break;
    case "warning":
	box.classList.add("toast-warning");
	break;
    case "error":
	box.classList.add("toast-error");
	break;
    }

    box.innerHTML = "<button id='INFO_BOX_CLOSE_BTN' class='btn btn-clear float-right'></button>" + info;
    document.getElementById("INFO_BOX_CLOSE_BTN").addEventListener("click", ()=>{
	box.style.display = "none";
    });
}

function clearState(box){
    box.classList.remove("toast-primary");
    box.classList.remove("toast-success");
    box.classList.remove("toast-warning");
    box.classList.remove("toast-error");
}
