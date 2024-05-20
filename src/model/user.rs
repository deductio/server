use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use crate::api::error::DeductResult;
use crate::model::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;
use crate::api::search::SearchResultGraph;
use crate::api::users::{AuthenticatedUser, ResponseUser};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub github_user_id: Option<String>,
    pub google_user_id: Option<String>,
    pub username: String,
    pub avatar: Option<String>,
    pub id: i64
}

impl User {
    pub async fn get_from_username(username: String, conn: &mut Connection<Db>) -> DeductResult<User> {
        Ok(users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .await?)
    }
}

#[derive(Serialize)]
pub struct UserPage {
    pub user: ResponseUser,
    pub graphs: Vec<SearchResultGraph>
}

impl UserPage {
    pub async fn get_user_with_offset(username: String, page: i64, maybe_user: Option<AuthenticatedUser>, conn: &mut Connection<Db>) 
        -> DeductResult<UserPage> 
    {
        let user = users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .await?;
    
        let graphs = KnowledgeGraph::belonging_to(&user)
            .select(KnowledgeGraph::as_select())
            .offset(page * 10)
            .limit(10)
            .load::<KnowledgeGraph>(conn)
            .await?;
    
        let res_user = ResponseUser {
            username: user.username.clone(),
            avatar: user.avatar.clone()
        };
    
        Ok(UserPage {
            user: ResponseUser {
                username: user.username.clone(),
                avatar: user.avatar.clone()
            },

            graphs: SearchResultGraph::get_likes(graphs
                .into_iter()
                .map(|graph| (graph, res_user.clone()))
                .collect(), maybe_user, conn).await?
        })
    }
}