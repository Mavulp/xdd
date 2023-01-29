use axum::extract::rejection::JsonRejection;
use axum::response::IntoResponse;
use idlib::{AuthorizeCookie, Has};

use anyhow::Context;
use axum::{extract::Path, Extension, Json};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_rusqlite::from_row;
use utoipa::ToSchema;

use std::sync::Arc;
use std::time::SystemTime;

use crate::error::Error;
use crate::util::non_empty_trimmed_str;
use crate::AppState;

/// Aliases are key value text replacements for links to images or other things that are difficult
/// to type out by hand. Each alias has a name which can be used as a key and content which is the
/// value. Other services may choose to use these in for example comments to replace occurances of
/// aliases with their content.
///
/// # Example
/// Quotes and hivefriends allow replacements in comments by prefixing with '!'.
/// ```
/// Alias: "fb", "foobar"  
/// Comment: "I like !fb" -> "I like foobar"
/// ```
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Alias {
    /// A short handle for users to easily remember the alias.
    #[schema(example = "funny.png")]
    pub name: String,

    /// The content of the alias which is to be used as the replacement.
    #[schema(example = "https://example.com/funny.png")]
    pub content: String,

    /// The username of the account who first created the alias.
    #[schema(example = "Alice")]
    pub author: String,

    /// A unix timestamp of when this alias was created.
    #[schema(example = 1670802822)]
    pub created_at: u64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct DbAlias {
    name: String,
    content: String,
    author: String,
    created_at: u64,
}

impl From<DbAlias> for Alias {
    fn from(alias: DbAlias) -> Self {
        Self {
            name: alias.name,
            content: alias.content,
            author: alias.author,
            created_at: alias.created_at,
        }
    }
}

/// Get a list of all aliases.
#[utoipa::path(
    get,
    path = "/api/alias",
    responses(
        (status = 200, description = "All aliases are returned.", body = [Alias]),
        (status = 302, description = "Redirects to hiveID if not authenticated."),
    )
)]
pub async fn get_aliases(
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

pub fn get_all(conn: &Connection) -> Result<Vec<Alias>, Error> {
    let mut stmt = conn
        .prepare(
            "SELECT
                name,
                content,
                author,
                created_at
            FROM aliases",
        )
        .context("Failed to prepare statement for alias query")?;

    let aliases = stmt
        .query_map(params![], |row| {
            Ok(Alias::from(from_row::<DbAlias>(row).unwrap()))
        })
        .context("Failed to query aliases")?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to collect aliases")?;

    Ok(aliases)
}

/// Get a alias by its name.
#[utoipa::path(
    get,
    path = "/api/alias/{name}",
    responses(
        (status = 200, description = "The content for the matching name is returned.", body = Alias),
        (status = 404, description = "No alias with that id exists."),
        (status = 302, description = "Redirects to hiveID if not authenticated."),
    ),
    params(
        ("name" = String, Path, description = "Name of the alias to query"),
    ),
)]
pub async fn get_alias_by_name(
    Path(name): Path<String>,
    AuthorizeCookie(_payload, maybe_token, ..): AuthorizeCookie<idlib::NoGroups>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    maybe_token
        .wrap_future(async move {
            state
                .db
                .call(move |conn| get_by_name(conn, name).map(|u| Json(u)))
                .await
        })
        .await
}

pub fn get_by_name(conn: &Connection, name: String) -> Result<Alias, Error> {
    let content = conn
        .query_row(
            "SELECT
                name,
                content,
                author,
                created_at
            FROM aliases
            WHERE name = $1",
            params![name],
            |row| Ok(Alias::from(from_row::<DbAlias>(row).unwrap())),
        )
        .optional()
        .context("Failed to query alias")?
        .ok_or(Error::NotFound)?;

    Ok(content)
}

type HasCreateAliases = Has<"create-aliases">;

/// Create alias from the body.
/// # Note
/// Requires `create-aliases` permission.
#[utoipa::path(
    post,
    path = "/api/alias",
    request_body = Alias,
    responses(
        (status = 200, description = "The alias was successfully created."),
        (status = 400, description = "One of the values sent in is invalid."),
        (status = 403, description = "User does not have the required permissions."),
        (status = 302, description = "Redirects to hiveID if not authenticated."),
    )
)]
pub async fn post_alias(
    AuthorizeCookie(payload, maybe_token, ..): AuthorizeCookie<HasCreateAliases>,
    Extension(state): Extension<Arc<AppState>>,
    request: Result<Json<Alias>, JsonRejection>,
) -> impl IntoResponse {
    maybe_token
        .wrap_future(async move {
            let Json(request) = request?;

            if request.name.is_empty() {
                return Err(Error::EmptyField("name"));
            }
            if request.content.is_empty() {
                return Err(Error::EmptyField("content"));
            }

            let now = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs();

            state
                .db
                .call(move |conn| {
                    conn.execute(
                        &format!(
                            "INSERT INTO aliases (name, content, author, created_at)
                            VALUES ($1, $2, $3, $4)"
                        ),
                        params![&request.name, &request.content, &payload.name, now],
                    )
                    .optional()
                })
                .await
                .context("Failed to insert alias")?;

            Ok::<_, Error>(())
        })
        .await
}

type HasEditAliases = Has<"edit-aliases">;

/// A list of fields that can be updated for an alias. To leave
/// fields as they are they can be skipped, set to null or set to a whitespace only string.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PutAlias {
    /// The content of the alias which is to be used as the replacement.
    /// # Note
    /// The input is trimmed and empty inputs are not updated.
    #[schema(example = "https://example.com/funny.png")]
    #[serde(default, deserialize_with = "non_empty_trimmed_str")]
    pub content: Option<String>,
}

/// Update alias for the specified alias name.
/// # Note
/// Requires either `edit-aliases` permission.
#[utoipa::path(
    put,
    path = "/api/alias/{name}",
    request_body = PutAlias,
    responses(
        (status = 200, description = "The alias was successfully updated."),
        (status = 400, description = "One of the values sent in is invalid."),
        (status = 403, description = "User does not have the required permissions."),
        (status = 302, description = "Redirects to hiveID if not authenticated."),
    ),
    params(
        ("name" = String, Path, description = "Name of the alias to update."),
    )
)]
pub async fn put_alias_by_name(
    Path(name): Path<String>,
    AuthorizeCookie(_payload, maybe_token, ..): AuthorizeCookie<HasEditAliases>,
    Extension(state): Extension<Arc<AppState>>,
    request: Result<Json<PutAlias>, JsonRejection>,
) -> impl IntoResponse {
    maybe_token
        .wrap_future(async move {
            let Json(request) = request?;

            let check_name = name.clone();
            let alias_exists = state
                .db
                .call(move |conn| {
                    conn.query_row(
                        "SELECT 1 FROM aliases
                        WHERE name = ?",
                        params![&check_name],
                        |row| Ok(from_row::<i64>(row).unwrap()),
                    )
                })
                .await
                .optional()
                .context("Failed to check if alias exists")?;

            if alias_exists.is_none() {
                return Err(Error::NotFound);
            }

            state
                .db
                .call(move |conn| {
                    conn.query_row(
                        &format!("UPDATE aliases SET content = ? WHERE name = ?"),
                        params![request.content, name],
                        |_| Ok(()),
                    )
                    .optional()
                })
                .await
                .context("Failed to update alias fields")?;

            Ok::<_, Error>(())
        })
        .await
}

type HasDeleteAliases = Has<"delete-aliases">;

/// Delete alias by its name.
/// # Note
/// Requires `delete-aliases`permission.
#[utoipa::path(
    delete,
    path = "/api/alias/{name}",
    responses(
        (status = 200, description = "The alias was successfully deleted."),
        (status = 404, description = "Alias with the specified name does not exist."),
        (status = 403, description = "User does not have the required permissions."),
        (status = 302, description = "Redirects to hiveID if not authenticated."),
    ),
    params(
        ("name" = String, Path, description = "Name of the alias to delete."),
    )
)]
pub async fn delete_alias_by_name(
    Path(name): Path<String>,
    AuthorizeCookie(_payload, maybe_token, ..): AuthorizeCookie<HasDeleteAliases>,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    maybe_token
        .wrap_future(async move {
            state
                .db
                .call(move |conn| {
                    conn.query_row(
                        &format!("DELETE FROM aliases WHERE name = ?"),
                        params![name],
                        |_| Ok(()),
                    )
                    .optional()
                })
                .await
                .context("Failed to delete alias")?;

            Ok::<_, Error>(())
        })
        .await
}
