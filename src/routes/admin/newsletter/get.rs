use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn newsletter_form() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Post Newsletter</title>
</head>
<body>
    Please enter the details for the new issue of your newsletter:
    <form method="post" action="/admin/newsletter">
        <label>
            Title
            <input
                type="text"
                placeholder="Title"
                name="title"
            >
        </label>
        <label>
            HTML Content
            <textarea
                placeholder="<p>HTML Content</p>"
                name="html_content"
                rows="20"
                cols="50"
            >
            </textarea>
        </label>
        <label>
            Text Content
            <textarea
                placeholder="Text Content"
                name="text_content"
                rows="20"
                cols="50"
            >
            </textarea>
        </label>
        <br>
        <button type="submit">Post Newsletter</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>
"#,
        )))
}
