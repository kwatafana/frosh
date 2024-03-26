window.addEventListener("load", async()=> {
    infoBox("😀 Fetching blog posts...", "primary");

    try {
	let blog_posts = await KongBlogAPI.get_all();
	let blogList = document.getElementById("BLOG_LIST");

	infoBox("😃 Blog posts have been fetched and loaded", "success");

	for(var i = 0; i < blog_posts.length; i++){
	    let img_url = "";
	    let overview = "";
	    let subtitle = "";

	    if(blog_posts[i].cover){
		img_url = blog_posts[i].cover;
	    }

	    if(blog_posts[i].overview){
		overview = blog_posts[i].overview;
	    }

	    if(blog_posts[i].subtitle){
		overview = blog_posts[i].subtitle;
	    }

	    blogList.innerHTML = blogList.innerHTML + `
<div class="card blog-card column col-3">
  <div class="card-image">
    <img src="/${img_url}" class="img-responsive">
  </div>
  <div class="card-header">
    <div class="card-title h5">${blog_posts[i].title}</div>
    <div class="card-subtitle text-gray">${subtitle}</div>
  </div>
  <div class="card-body">
    ${overview}
  </div>
  <div class="card-footer">
    <div class="btn-group btn-group-block">
      <a href='./edit-blog.html?id=${blog_posts[i].id}' class="btn btn-primary">Edit</a>
      <a href='./view-blog.html?id=${blog_posts[i].id}' class="btn btn">View</a>
      <button class="btn btn-error deleteBtns" id='${blog_posts[i].id}'>Delete</button>
    </div>
  </div>
</div>`;
	}

	const deleteBtns = document.getElementsByClassName("deleteBtns");

	for(var j = 0; j < deleteBtns.length; j++){
	    let id = parseInt(deleteBtns[j].id);
	    deleteBtns[j].addEventListener("click", async()=>{
		let blog_posts = await KongBlogAPI.delete_by_id(id);
		infoBox("🚮 Post has been deleted", "success");
	    });
	}
    } catch(e){
	infoBox(e, 'error');
    }
});
