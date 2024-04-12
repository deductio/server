// @generated automatically by Diesel CLI.

diesel::table! {
    extensions (source, destination) {
        source -> Uuid,
        destination -> Uuid,
    }
}

diesel::table! {
    knowledge_graphs (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        author -> Int8,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    objective_prerequisites (knowledge_graph_id, topic, objective) {
        knowledge_graph_id -> Uuid,
        topic -> Int8,
        objective -> Int8,
        topic_to_objective -> Bool,
    }
}

diesel::table! {
    objectives (id) {
        id -> Int8,
        title -> Text,
        description -> Text,
    }
}

diesel::table! {
    progress (user_id, graph) {
        graph -> Uuid,
        user_progress -> Array<Nullable<Int4>>,
        user_id -> Int8,
    }
}

diesel::table! {
    requirements (id) {
        source -> Int8,
        destination -> Int8,
        knowledge_graph_id -> Uuid,
        id -> Int8,
    }
}

diesel::table! {
    topics (id) {
        knowledge_graph_id -> Uuid,
        title -> Text,
        id -> Int8,
        subject -> Text,
        content -> Jsonb,
    }
}

diesel::table! {
    users (id) {
        github_user_id -> Nullable<Text>,
        google_user_id -> Nullable<Text>,
        username -> Text,
        avatar -> Nullable<Text>,
        id -> Int8,
    }
}

diesel::joinable!(knowledge_graphs -> users (author));
diesel::joinable!(objective_prerequisites -> knowledge_graphs (knowledge_graph_id));
diesel::joinable!(objective_prerequisites -> objectives (objective));
diesel::joinable!(objective_prerequisites -> topics (topic));
diesel::joinable!(progress -> knowledge_graphs (graph));
diesel::joinable!(progress -> users (user_id));
diesel::joinable!(requirements -> knowledge_graphs (knowledge_graph_id));
diesel::joinable!(topics -> knowledge_graphs (knowledge_graph_id));

diesel::allow_tables_to_appear_in_same_query!(
    extensions,
    knowledge_graphs,
    objective_prerequisites,
    objectives,
    progress,
    requirements,
    topics,
    users,
);
