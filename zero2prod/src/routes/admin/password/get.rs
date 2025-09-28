use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;

use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

use std::fmt::Write;

pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    }
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        // println!("m.content: {:?}", m.content());
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    // println!("msg_html: {:?}", msg_html);
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8"/>
        <title>Change Password</title>
    </head>
    <body>
        {msg_html}
        <form action="/admin/password" method="post">
            <label>Current password
                <input
                    type="password"
                    name="current_password"
                    placeholder="Enter current password"
                />
            </label>
            <br>
            <label>New password
                <input
                    type="password"
                    name="new_password"
                    placeholder="Enter new password"
                />
            </label>
            <br>
            <label>Confirm new password
                <input
                    type="password"
                    name="new_password_check"
                    placeholder="Type the new password again"
                />
            </label>
            <br>
            <button type="submit">Change password</button>
        </form>
        <p><a href="/admin/dashboard"><- Back</a></p>
    </body>
</html>"#,
        )))
}
