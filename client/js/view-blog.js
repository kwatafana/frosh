window.addEventListener("load", async()=> {
    infoBox("😀 Fetching blog post... please wait", "primary");

    try {
	const queryString = window.location.search;
	const urlParams = new URLSearchParams(queryString);
	const id = urlParams.get('id');
	const blogPost = await KongBlogAPI.get_by_id(id);
	var blogTitle = document.getElementById("BLOG_TITLE");
	var blogSubTitle = document.getElementById("SUB_TITLE");
	var overview = document.getElementById("OVERVIEW");
	var author = document.getElementById("AUTHOR");
	var cover = document.getElementById("COVER");
	var content = document.getElementById("CONTENT");
	var date = document.getElementById("DATE");

	infoBox("😃 Blog post has been fetched and loaded", "success");
	blogTitle.innerHTML = blogPost.title;
	content.innerHTML = blogPost.content;
	date.innerHTML = blogPost.date;

	if (blogPost.subtitle){
	    blogSubTitle.innerHTML = blogPost.subtitle;
	}

	if (blogPost.overview){
	    overview.innerHTML = blogPost.overview;
	}

	if(blogPost.author){
	    author.innerHTML = blogPost.author;
	}

	if(blogPost.cover){
	    cover.src = `/${blogPost.cover}`;
	}
    } catch(e){
	infoBox(e, 'error');
    }
});
