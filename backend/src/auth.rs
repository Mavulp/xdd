/// Consumes the response from the IDP and stores the received token as a cookie in the client's
/// browser.
/// # Note
/// This will also run service specific code like creating a user account on first login.
#[utoipa::path(
    get,
    path = "/api/auth/authorize",
    responses(
        (status = 302, description = "Succes, tries to set cookie and redirects."),
        (status = 403, description = "Missing or invalid parameter."),
    ),
    security(()),
)]
async fn _authorize_dummy() {}

/// I'm not sure what this does, the code just returns a success?
#[utoipa::path(
    post,
    path = "/api/auth/revoke",
    responses(
        (status = 200, description = "Nothing was done?"),
    ),
    security(()),
)]
async fn _revoke_dummy() {}

/// Responds with a redirect to `/` and tells the browser to unset the auth cookie.
#[utoipa::path(
    post,
    path = "/api/auth/logout",
    responses(
        (status = 302, description = "Success tries to unset cookie and redirects."),
    ),
    security(()),
)]
async fn _logout_dummy() {}
