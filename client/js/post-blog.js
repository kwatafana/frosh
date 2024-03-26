document.getElementById("POST_BLOG_BTN").addEventListener("click", async () =>{
    let titleInput = document.getElementById("TITLE").value;
    let subtitleInput = document.getElementById("SUBTITLE").value;
    let overviewInput = document.getElementById("OVERVIEW").value;
    let authorInput = document.getElementById("AUTHOR").value;
    let coverInput = document.getElementById("COVER");
    let contentInput = document.getElementById("CONTENT").value;
    let subtitle = null;
    let overview = null;
    let author = null;
    let cover = null;

    infoBox("📰 posting your blog post, please wait...", "primary");

    if(subtitleInput != ""){
	subtitle = subtitleInput;
    }

    if(overviewInput != ""){
	overview = overviewInput;
    }

    if(authorInput != ""){
	author = authorInput;
    }

    if(coverInput.files[0]){
	cover = coverInput.files[0];
    }

    if(contentInput == "" || titleInput == ""){
	infoBox("🙄 The title and content cannot be empty", "error");
    } else{
	try {
	    let input = new KongBlogInput(titleInput, subtitle, overview, author, cover, contentInput);
	    await KongBlogAPI.post_blog(input);

	    infoBox("😃 Blog post has been published", "success");
	    document.location.href = '/portal/dashboard.html';
	} catch(e){
	    infoBox(e, "error");
	}
    }
});
