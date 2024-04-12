use rocket_db_pools::Connection;
use crate::error::DeductResult;
use crate::model::user;
use crate::model::user::User;
use crate::model::Db;
use crate::model::KnowledgeGraph;
use crate::schema::knowledge_graphs;
use crate::schema::users;
use crate::model::knowledge_graph::SearchResultGraph;
use rocket_db_pools::diesel::prelude::*;

#[derive(Serialize)]
pub struct ResponseUser {
    pub username: String,
    pub avatar: Option<String>
}

#[derive(Serialize)]
pub struct UserPage {
    pub user: ResponseUser,
    pub graphs: Vec<SearchResultGraph>
}

impl UserPage {
    pub async fn get_user_with_offset(username: String, page: i64, conn: &mut Connection<Db>) -> DeductResult<UserPage> {
        let user = users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
            .await?;
    
        let graphs = KnowledgeGraph::belonging_to(&user)
            .select((knowledge_graphs::id, knowledge_graphs::name, knowledge_graphs::description))
            .offset(page * 10)
            .limit(10)
            .load::<(uuid::Uuid, String, String)>(conn)
            .await?;
    
    
        Ok(UserPage {
            user: ResponseUser {
                username: user.username.clone(),
                avatar: user.avatar.clone()
            },

            graphs: graphs
                .iter()
                .map(|graph| SearchResultGraph { author: user.username.clone(), id: graph.0, name: graph.1.clone(), description: graph.2.clone() } )
                .collect()
        })
    }
}