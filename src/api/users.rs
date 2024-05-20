use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Request, Outcome};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket_oauth2::{OAuth2, TokenResponse};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;

use crate::model::user::UserPage;
use crate::model::{Db, User};
use crate::api::error::DeductResult;

pub struct AuthenticatedUser {
    pub name: String,
    pub db_id: i64
}

const PRIVATE_ALPHA: bool = true;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<AuthenticatedUser, ()> {
        let cookies = request.cookies();

        let id = cookies
            .get_private("id")
            .and_then(|c| c.value().parse().ok());

        let name = cookies
            .get("name")
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
    avatar_url: Option<String>
}

#[get("/github")]
pub fn github_login(oauth2: OAuth2<GitHubInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["user:read"]).unwrap()
}

#[get("/github")]
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
        
            cookies.add(
                Cookie::build(("name", user.username))
                    .same_site(SameSite::Lax)
                    .build()
            );

            match user.avatar {
                Some(avatar_url) => {
                    cookies.add(
                        Cookie::build(("avatar", avatar_url))
                            .same_site(SameSite::Lax)
                            .build()
                    )
                },
                None => ()
            };
        },
        Err(err) => match err {
            // we need to register this user!
            diesel::NotFound => {
                if !PRIVATE_ALPHA {
                    diesel::insert_into(users).values((
                        username.eq(user_info.name),
                        github_user_id.eq(user_info.id.to_string()),
                        avatar.eq(user_info.avatar_url)
                    ))
                    .execute(&mut conn)
                    .await?;
                } else {
                    return Err(err.into());
                }

            },
            _ => {
                return Err(err.into());
            }
        }
    }

    Ok(Redirect::to("http://localhost:5173/"))
}

#[derive(Serialize, Clone)]
pub struct ResponseUser {
    pub username: String,
    pub avatar: Option<String>
}

impl From<User> for ResponseUser {
    fn from(user: User) -> ResponseUser {
        ResponseUser {
            username: user.username,
            avatar: user.avatar
        }
    }
}

#[get("/<username>?<offset>")]
pub async fn view_user(username: String, mut conn: Connection<Db>, maybe_user: Option<AuthenticatedUser>, offset: Option<i64>) 
    -> DeductResult<Json<UserPage>> 
{
    Ok(Json(UserPage::get_user_with_offset(username, offset.unwrap_or(0), maybe_user, &mut conn).await?))
}

#[get("/")]
pub fn logout(_user: AuthenticatedUser, cookies: &CookieJar<'_>,) -> Redirect {

    cookies.remove("name");
    cookies.remove("avatar");
    cookies.remove_private("id");

    Redirect::to("http://localhost:5173/")

}