use crate::{META, STYLES};

pub fn login() -> String {
    let nav =
        std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/GuestNav.html").unwrap();
    let info_box =
        std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/InfoBox.html").unwrap();
    let login_form =
        std::fs::read_to_string("/home/kaindume/26/frosh/client/widgets/LoginForm.html").unwrap();
    format!(
        "<!DOCTYPE html>
<html lang='en'>
  <head>
    {META}
    <title>Login</title>
    {STYLES}
  </head>
  <body>    
    {nav}
    <div class='col-6 col-mx-auto pt-2 kontainer'>
      <h3 class='pt-2'><i class='fa fa-lock'></i> Account Login</h3>
      {login_form}
    </div>
    {info_box}
    <script src='/js/kong.js'></script>
    <script src='/portal/js/info-box.js'></script>
    <script src='/portal/js/login.js'></script>
  </body>
</html>"
    )
}
