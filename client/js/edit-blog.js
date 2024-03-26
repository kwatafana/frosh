window.addEventListener("load", async()=> {
    try{
	const queryString = window.location.search;
	const urlParams = new URLSearchParams(queryString);
	const id = urlParams.get('id');
	const blogPost = await KongBlogAPI.get_by_id(id);

	document.getElementById("TITLE").value = blogPost.title;
	document.getElementById("CONTENT").value = blogPost.content;

	if(blogPost.subtitle){
	    document.getElementById("SUBTITLE").value = blogPost.subtitle;
	}

	if(blogPost.overview){
	    document.getElementById("OVERVIEW").value = blogPost.overview;
	}

	if(blogPost.author){
	    document.getElementById("AUTHOR").value = blogPost.author;
	}

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

	    await infoBox("📰 updating your blog post, please wait...", "primary");

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

	    if(document.getElementById("COVER") == "" || titleInput == ""){
		infoBox("🙄 The title and content cannot be empty", "error");
	    } else{
		try{
		    let input = new KongBlogInput(titleInput, subtitle, overview, author, cover, contentInput);
		    await KongBlogAPI.update(id, input);

		    infoBox("😃 Blog post has been updated", "success");
		    document.location.href = '/portal/dashboard.html';
		} catch(e){
		    infoBox(e, "error");
		}
	    }
	});

    } catch(e){
	infoBox(e, "error");
    }

});
