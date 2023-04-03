use axum::response::IntoResponse;
use idlib::AuthorizeCookie;

use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_rusqlite::from_row;
use tracing::{debug, info};
use ts_rs::TS;
use utoipa::ToSchema;

use std::sync::Arc;
use std::time::SystemTime;

use crate::error::Error;
use crate::AppState;

#[derive(Debug, Serialize, TS, ToSchema)]
#[ts(export, export_to = "../frontend/src/types/")]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[schema(example = "alice")]
    pub username: String,

    #[schema(example = 1670802822)]
    pub created_at: u64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct DbUser {
    username: String,
    created_at: u64,
}

impl From<DbUser> for User {
    fn from(user: DbUser) -> Self {
        Self {
            username: user.username,
            created_at: user.created_at,
        }
    }
}

pub fn get_all(conn: &Connection) -> Result<Vec<User>, Error> {
    let query = "SELECT
                    username,
                    created_at
                FROM users"
        .to_string();

    let params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    let mut stmt = conn
        .prepare(&query)
        .context("Failed to prepare statement for user query")?;
    let users = stmt
        .query_map(rusqlite::params_from_iter(params.iter()), |row| {
            Ok(User::from(from_row::<DbUser>(row).unwrap()))
        })
        .context("Failed to query users")?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to collect users")?;

    Ok(users)
}

/// Get a list of all users.
#[utoipa::path(
    get,
    path = "/api/user",
    responses(
        (status = 200, description = "User data is returned", body = [User]),
        (status = 302, description = "Redirects to hiveID if not authenticated"),
    )
)]
pub async fn get_users(
    AuthorizeCookie(_payload, maybe_token, ..): AuthorizeCookie<idlib::NoGroups>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    maybe_token
        .wrap_future(async move {
            state
                .db
                .call(move |conn| get_all(conn).map(|u| Json(u)))
                .await
        })
        .await
}

/// Get user by username.
#[utoipa::path(
    get,
    path = "/api/user/{username}",
    responses(
        (status = 200, description = "User data is returned", body = User),
        (status = 404, description = "User does not exist"),
        (status = 302, description = "Redirects to hiveID if not authenticated"),
    ),
    params(
        ("username" = String, Path, description = "Username of the user to query"),
    )
)]
pub async fn get_user_by_username(
    Path(username): Path<String>,
    AuthorizeCookie(_payload, maybe_token, ..): AuthorizeCookie<idlib::NoGroups>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    maybe_token
        .wrap_future(async move {
            let user = state
                .db
                .call(move |conn| {
                    conn.query_row(
                        "SELECT
                            username,
                            created_at
                        FROM users WHERE username = ?1",
                        params![username],
                        |row| Ok(Json(User::from(from_row::<DbUser>(row).unwrap()))),
                    )
                    .optional()
                })
                .await
                .context("Failed to query users")?;

            user.ok_or(Error::NotFound)
        })
        .await
}

pub async fn create_user_if_missing(
    db: tokio_rusqlite::Connection,
    name: String,
) -> anyhow::Result<()> {
    db.call(move |conn| {
        if user_exists(&name, conn)? {
            debug!("User already existed for {name}");
        } else {
            let now = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs();

            conn.execute(
                "INSERT INTO users (username, created_at) VALUES (?1, ?2)",
                params![name, now],
            )?;
            info!("Created user for user {name}");
        }

        Ok::<(), anyhow::Error>(())
    })
    .await?;

    Ok(())
}

pub fn user_exists(username: &str, conn: &Connection) -> anyhow::Result<bool> {
    let result = conn.query_row(
        "SELECT 1 FROM users WHERE username = ?1",
        params![username],
        |_| Ok(()),
    );

    if matches!(result, Err(rusqlite::Error::QueryReturnedNoRows)) {
        Ok(false)
    } else {
        result?;

        Ok(true)
    }
}
