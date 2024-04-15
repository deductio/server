use rocket::form::Form;
use rocket::serde::json::Json;
use crate::api::error::DeductResult;
use rocket_db_pools::Connection;
use crate::model::*;

/// Represents a result in a search for graphs. Provides the ID of the graph (to
/// access the link), as well as the title, description, and author of the graph.
#[derive(Serialize)]
pub struct SearchResultGraph {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub author: String,
    pub last_modified: std::time::SystemTime
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

#[post("/?<offset>", data = "<data>")]
pub async fn search_graph(data: Form<GraphSearchForm>, offset: Option<i64>, mut conn: Connection<Db>) -> DeductResult<Json<Vec<SearchResultGraph>>> {
    let offset = offset.unwrap_or(0);

    Ok(Json(KnowledgeGraph::search(data.search.clone(), offset, &mut conn).await?))
}
