pub const STYLES: &str = r#"<link rel='stylesheet' href='./css/vendor/spectre.min.css'>
    <link rel='stylesheet' href='./css/vendor/spectre-exp.min.css'>
    <link rel='stylesheet' href='./css/vendor/spectre-icons.min.css'>
   <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">
    <link rel='stylesheet' href='./css/main.css'>"#;

pub const META: &str = r#"
    <meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <meta http-equiv='X-UA-Compatible' content='ie=edge'>
"#;

mod account;
mod blog;
mod dashboard;
mod login;

fn main() {
    let create_account_html = account::create_account();
    let dashboard_html = dashboard::dashboard();
    let login_html = login::login();
    let blog_dashboard_html = blog::blog_dashboard();
    let post_blog_html = blog::post_blog();
    let view_blog_html = blog::view_blog();
    let edit_blog_html = blog::edit_blog();

    std::fs::write(".www/create-account.html", create_account_html.as_bytes()).unwrap();
    std::fs::write(".www/dashboard.html", dashboard_html.as_bytes()).unwrap();
    std::fs::write(".www/login.html", login_html.as_bytes()).unwrap();
    std::fs::write(".www/post-blog.html", post_blog_html.as_bytes()).unwrap();
    std::fs::write(".www/blog-dashboard.html", blog_dashboard_html.as_bytes()).unwrap();
    std::fs::write(".www/view-blog.html", view_blog_html.as_bytes()).unwrap();
    std::fs::write(".www/edit-blog.html", edit_blog_html.as_bytes()).unwrap();
}
