use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Request, Outcome};
use rocket::response::Redirect;
use rocket_oauth2::{OAuth2, TokenResponse};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;

use crate::model::{Db, User};
use crate::api::error::DeductResult;

pub struct AuthenticatedUser {
    pub name: String,
    pub db_id: i64
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<AuthenticatedUser, ()> {
        let cookies = request.cookies();

        let id = cookies
            .get_private("id")
            .and_then(|c| c.value().parse().ok());

        let name = cookies
            .get_private("name")
            .and_then(|c| c.value().parse().ok());
            
        name.zip(id)
            .map(|(name, id)| AuthenticatedUser {
                name: name,
                db_id: id
            })
            .or_forward(Status::Unauthorized)

    }
}

#[derive(Deserialize)]
pub struct GitHubInfo {
    name: String,
    id: i64,
    avatar_url: String
}

#[get("/login/github")]
pub fn github_login(oauth2: OAuth2<GitHubInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["user:read"]).unwrap()
}

#[get("/auth/github")]
pub async fn github_callback(token: TokenResponse<GitHubInfo>, cookies: &CookieJar<'_>, mut conn: Connection<Db>) 
    -> DeductResult<Redirect>
{

    use crate::schema::users::dsl::*;

    let user_info: GitHubInfo = reqwest::Client::builder()
        .build()?
        .get("https://api.github.com/user")
        .header(AUTHORIZATION, format!("token {}", token.access_token()))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "rocket_oauth2 deduct-io")
        .send()
        .await?
        .json()
        .await?;

    let db_user = users
        .filter(github_user_id.eq(user_info.id.to_string()))
        .first::<User>(&mut conn)
        .await;

    match db_user {
        // user is registered
        Ok(user) => {
            cookies.add_private(
                Cookie::build(("id", user.id.to_string()))
                    .same_site(SameSite::Lax)
                    .build()
            );
        
            cookies.add_private(
                Cookie::build(("name", user.username))
                    .same_site(SameSite::Lax)
                    .build()
            );
        },
        Err(err) => match err {
            // we need to register this user!
            diesel::NotFound => {
                diesel::insert_into(users).values((
                    username.eq(user_info.name),
                    github_user_id.eq(user_info.id.to_string()),
                    avatar.eq(user_info.avatar_url)
                ))
                .execute(&mut conn)
                .await?;

            },
            _ => {
                return Err(err.into());
            }
        }
    }

    Ok(Redirect::to("/"))
}