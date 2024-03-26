use crate::{META, STYLES};

pub fn dashboard() -> String {
    let nav = std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/Nav.html").unwrap();
    format!(
            "<!DOCTYPE html>
<html lang='en'>
  <head>
    <title>Dashboard</title>
    {STYLES}
  </head>
  <body>
    {nav}

<div class='empty'>
  <div class='empty-icon'>
    <i class='icon icon-flag'></i>
  </div>
  <p class='empty-title h5'>You have no new messages</p>
  <p class='empty-subtitle'>Click the button to create a new blog post.</p>
  <div class='empty-action'>
    <a href='/portal/blog-dashboard.html' class='btn btn-primary btn-lg'><i class='fa fa-newspaper-o'></i> Blog</a>
  </div>
</div>
  </body>
</html>"
        )
}
