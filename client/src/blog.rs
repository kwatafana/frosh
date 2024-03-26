use crate::{META, STYLES};

pub fn blog_dashboard() -> String {
    let nav = std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/Nav.html").unwrap();
    let blog_dashboard =
        std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/BlogDashboard.html")
            .unwrap();
    let info_box =
        std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/InfoBox.html").unwrap();
    format!(
        "<!DOCTYPE html>
<html lang='en'>
  <head>
    <title>Blog Dashboard</title>
    {STYLES}
  </head>
  <body>
    {nav}
    <div class='col-10 col-mx-auto pt-2 kontainer'>
     <div>
       <a href='./post-blog.html' class='btn btn-primary' style='float:right'>
         <i class='icon icon-plus'></i>
       </a>
     </div>
      <ul class='breadcrumb'>
       <li class='breadcrumb-item'>
         <a href='./dashboard.html'>Dashboard</a>
       </li>
      <li class='breadcrumb-item'>
        <a class='text-dark'>Blog</a>
      </li>
     </ul>
     {blog_dashboard}
    </div>
    {info_box}
    <script src='/portal/js/kong.js'></script>
    <script src='/portal/js/info-box.js'></script>
    <script src='/portal/js/blog-dashboard.js'></script>
  </body>
</html>"
    )
}

pub fn post_blog() -> String {
    let nav = std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/Nav.html").unwrap();
    let info_box =
        std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/InfoBox.html").unwrap();
    let blog_form =
        std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/BlogForm.html").unwrap();
    format!(
        "<!DOCTYPE html>
<html lang='en'>
  <head>
    {META}
    <title>Blog Post</title>
    {STYLES}
  </head>
  <body>    
    {nav}
    <div class='col-6 col-mx-auto pt-2 kontainer'>
      <h3 class='pt-2'><i class='fa fa-newspaper-o'></i> Ceate a blog post</h3>
      {blog_form}
    </div>
    {info_box}
    <script src='/portal/js/kong.js'></script>
    <script src='/portal/js/info-box.js'></script>
    <script src='/portal/js/post-blog.js'></script>
  </body>
</html>"
    )
}

pub fn view_blog() -> String {
    let nav = std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/Nav.html").unwrap();
    let view_blog =
        std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/ViewBlog.html").unwrap();
    let info_box =
        std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/InfoBox.html").unwrap();
    format!(
        "<!DOCTYPE html>
<html lang='en'>
  <head>
    {META}
    <title>Blog Post</title>
    {STYLES}
  </head>
  <body>
    {nav}
    <div class='col-6 col-mx-auto pt-2 kontainer'>
      {view_blog}
    </div>
    {info_box}
    <script src='/portal/js/kong.js'></script>
    <script src='/portal/js/info-box.js'></script>
    <script src='/portal/js/view-blog.js'></script>
  </body>
</html>"
    )
}

pub fn edit_blog() -> String {
    let nav = std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/Nav.html").unwrap();
    let edit_blog =
        std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/BlogForm.html").unwrap();
    let info_box =
        std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/InfoBox.html").unwrap();
    format!(
        "<!DOCTYPE html>
<html lang='en'>
  <head>
    {META}
    <title>Edit Blog Post</title>
    {STYLES}
  </head>
  <body>
    {nav}
    <div class='col-6 col-mx-auto pt-2 kontainer'>
      {edit_blog}
    </div>
    {info_box}
    <script src='/portal/js/kong.js'></script>
    <script src='/portal/js/info-box.js'></script>
    <script src='/portal/js/edit-blog.js'></script>
  </body>
</html>"
    )
}
