// @generated automatically by Diesel CLI.

diesel::table! {
    extensions (source, destination) {
        source -> Uuid,
        destination -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    knowledge_graphs (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        author -> Int8,
        last_modified -> Date,
        tsv_name_desc -> Tsvector,
        like_count -> Int4,
    }
}

diesel::table! {
    learning_map_goals (learning_map_id, topic_id) {
        learning_map_id -> Int8,
        topic_id -> Int8,
    }
}

diesel::table! {
    learning_maps (id) {
        id -> Int8,
        user_id -> Int8,
        title -> Text,
    }
}

diesel::table! {
    likes (knowledge_graph_id, user_id) {
        knowledge_graph_id -> Uuid,
        user_id -> Int8,
        like_date -> Date,
    }
}

diesel::table! {
    notifications (user_id, content) {
        user_id -> Int8,
        notif_time -> Timestamp,
        content -> Jsonb,
        dismissal_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    objective_prerequisites (knowledge_graph_id, topic, objective) {
        knowledge_graph_id -> Uuid,
        topic -> Int8,
        objective -> Int8,
        suggested_topic -> Int8,
        suggested_graph -> Uuid,
    }
}

diesel::table! {
    objective_satisfiers (knowledge_graph_id, topic) {
        knowledge_graph_id -> Uuid,
        objective -> Int8,
        topic -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    objectives (id) {
        id -> Int8,
        title -> Text,
        description -> Text,
        tsv_title_desc -> Tsvector,
        author -> Nullable<Int8>,
    }
}

diesel::table! {
    progress (user_id, topic) {
        user_id -> Int8,
        knowledge_graph_id -> Uuid,
        topic -> Int8,
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
    user_objective_progress (user_id, objective_id) {
        user_id -> Int8,
        objective_id -> Int8,
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
diesel::joinable!(learning_map_goals -> learning_maps (learning_map_id));
diesel::joinable!(learning_map_goals -> topics (topic_id));
diesel::joinable!(learning_maps -> users (user_id));
diesel::joinable!(likes -> knowledge_graphs (knowledge_graph_id));
diesel::joinable!(likes -> users (user_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(objective_prerequisites -> knowledge_graphs (knowledge_graph_id));
diesel::joinable!(objective_prerequisites -> objectives (objective));
diesel::joinable!(objective_satisfiers -> objectives (objective));
diesel::joinable!(objectives -> users (author));
diesel::joinable!(progress -> users (user_id));
diesel::joinable!(requirements -> knowledge_graphs (knowledge_graph_id));
diesel::joinable!(topics -> knowledge_graphs (knowledge_graph_id));
diesel::joinable!(user_objective_progress -> objectives (objective_id));
diesel::joinable!(user_objective_progress -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    extensions,
    knowledge_graphs,
    learning_map_goals,
    learning_maps,
    likes,
    notifications,
    objective_prerequisites,
    objective_satisfiers,
    objectives,
    progress,
    requirements,
    topics,
    user_objective_progress,
    users,
);
