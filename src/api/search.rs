use rocket::form::Form;
use rocket::serde::json::Json;
use crate::api::error::DeductResult;
use crate::users::ResponseUser;
use rocket_db_pools::Connection;
use crate::model::*;
use crate::users::AuthenticatedUser;
use std::collections::HashSet;
use crate::schema::likes;
use rocket_db_pools::diesel::prelude::*;

/// Represents a result in a search for graphs. Provides the ID of the graph (to
/// access the link), as well as the title, description, and author of the graph.
#[derive(Serialize)]
pub struct SearchResultGraph {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub user: ResponseUser,
    pub last_modified: chrono::NaiveDate,
    pub like_count: i32,
    pub liked: bool
}

impl SearchResultGraph {
    pub async fn get_likes<U: Into<ResponseUser>>(results: Vec<(KnowledgeGraph, U)>, maybe_user: Option<AuthenticatedUser>, conn: &mut Connection<Db>) -> DeductResult<Vec<SearchResultGraph>> {
        Ok(if let Some(authenticated_user) = maybe_user {
            let like_candidates: Vec<uuid::Uuid> = results.iter().map(|(graph, _)| graph.id).collect();
    
            let user_likes: HashSet<uuid::Uuid> = likes::table
                .select(likes::knowledge_graph_id)
                .filter(likes::user_id.eq(authenticated_user.db_id).and(likes::knowledge_graph_id.eq_any(like_candidates)))
                .load(conn)
                .await?
                .into_iter()
                .collect();
    
            results.into_iter().map(|(graph, user)| SearchResultGraph {
                id: graph.id,
                name: graph.name,
                description: graph.description,
                user: user.into(),
                last_modified: graph.last_modified,
                like_count: graph.like_count,
                liked: user_likes.contains(&graph.id)
            })
            .collect()
        } else {
            results.into_iter().map(|(graph, user)| SearchResultGraph {
                id: graph.id,
                name: graph.name,
                description: graph.description,
                user: user.into(),
                last_modified: graph.last_modified,
                like_count: graph.like_count,
                liked: false
            })
            .collect()
        })
    }
}

#[derive(FromFormField)]
pub enum GraphSearchSorting {
    HighestRating,
    LastModified,
    BestMatch
}

#[derive(FromForm)]
pub struct GraphSearchForm {
    pub search: String,
    pub order: GraphSearchSorting
}

#[derive(FromFormField)]
pub enum TrendingRange {
    Day,
    Week,
    Month,
    AllTime
}

impl<'r> rocket::request::FromParam<'r> for TrendingRange {
    type Error = &'r str;

    fn from_param(string: &'r str) -> Result<Self, Self::Error> {
        match string {
            "day" => Ok(TrendingRange::Day),
            "week" => Ok(TrendingRange::Week),
            "month" => Ok(TrendingRange::Month),
            "all_time" => Ok(TrendingRange::AllTime),
            _ => Err("not acceptable")
        }
    }
}


#[post("/?<offset>", data = "<data>", rank = 2)]
pub async fn search_graph(data: Form<GraphSearchForm>, offset: Option<i64>, maybe_user: Option<AuthenticatedUser>, mut conn: Connection<Db>) 
-> DeductResult<Json<Vec<SearchResultGraph>>> 
{
    let offset = offset.unwrap_or(0);

    Ok(Json(KnowledgeGraph::search(data.search.clone(), offset, maybe_user, &mut conn).await?))
}

#[get("/?<timerange>")]
pub async fn get_trending_graphs(timerange: Option<TrendingRange>, maybe_user: Option<AuthenticatedUser>, mut conn: Connection<Db>) 
    -> DeductResult<Json<Vec<SearchResultGraph>>> 
{
    let range = timerange.unwrap_or(TrendingRange::Day);

    Ok(Json(KnowledgeGraph::trending(range, 10, maybe_user, &mut conn).await?))
}


