use axum::{
    http::StatusCode,
    routing::{delete, get, post, put, Router},
    Extension,
};
use futures::FutureExt;
use idlib::{AuthCallback, IdpClient, SecretKey, Variables};
use rusqlite_migration::{Migrations, M};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use std::path::Path;
use std::sync::Arc;

pub mod util;

mod account;
mod alias;
mod auth;
mod error;
mod user;

pub struct AppState {
    db: tokio_rusqlite::Connection,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        account::get_login,
        account::get_settings,
        account::put_settings,
        health,
        user::get_users,
        user::get_user_by_username,
        alias::get_aliases,
        alias::post_alias,
        alias::get_alias_by_name,
        alias::put_alias_by_name,
        alias::delete_alias_by_name,
        auth::_authorize_dummy,
        auth::_revoke_dummy,
        auth::_logout_dummy
    ),
    components(schemas(
        user::User,
        alias::Alias,
        alias::PostAlias,
        alias::PutAlias,
        alias::AliasType,
        account::Settings,
        account::PutSettings
    )),
    modifiers(&SecurityAddon),
    security(
        ("hiveid-jwt-cookie" = []),
        ("hiveid-jwt-header" = []),
    ),
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "hiveid-jwt-cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("__auth"))),
            );
            components.add_security_scheme(
                "hiveid-jwt-header",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
    }
}

pub async fn api_route(db: tokio_rusqlite::Connection) -> anyhow::Result<Router> {
    let secret_key = SecretKey::from_env()?;
    let variables = Variables::from_env()?;

    let idp_client = IdpClient::default();

    let cdb = db.clone();
    let auth_callback = AuthCallback(Arc::new(Box::new(move |name| {
        crate::user::create_user_if_missing(cdb.clone(), name).boxed()
    })));

    Ok(Router::new()
        .merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route("/api/health", get(health))
        .route("/api/account/settings", get(account::get_settings))
        .route("/api/account/settings", put(account::put_settings))
        .route("/api/account/login", get(account::get_login))
        .route("/api/user", get(user::get_users))
        .route("/api/user/:username", get(user::get_user_by_username))
        .route("/api/alias", get(alias::get_aliases))
        .route("/api/alias", post(alias::post_alias))
        .route("/api/alias/:name", get(alias::get_alias_by_name))
        .route("/api/alias/:name", put(alias::put_alias_by_name))
        .route("/api/alias/:name", delete(alias::delete_alias_by_name))
        .nest(
            "/api/auth",
            idlib::api_route(idp_client, Some(auth_callback)),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(Arc::new(AppState { db })))
        .layer(Extension(IdpClient::default()))
        .layer(Extension(secret_key))
        .layer(Extension(Arc::new(variables))))
}

/// Simple check to see if the service is still up.
#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Service is up"),
    )
)]
async fn health() -> StatusCode {
    StatusCode::OK
}

pub(crate) const MIGRATIONS: [M; 1] = [M::up(include_str!("../migrations/001_initial.sql"))];

pub async fn setup_database(path: &Path) -> anyhow::Result<tokio_rusqlite::Connection> {
    let db = tokio_rusqlite::Connection::open(path).await?;

    let migrations = Migrations::new(MIGRATIONS.to_vec());

    db.call(move |conn| {
        conn.pragma_update(None, "foreign_keys", &"OFF")?;
        migrations.to_latest(conn)?;
        conn.pragma_update(None, "foreign_keys", &"ON")?;

        Ok::<_, anyhow::Error>(())
    })
    .await?;

    Ok(db)
}
