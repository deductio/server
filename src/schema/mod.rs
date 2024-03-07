// @generated automatically by Diesel CLI.

diesel::table! {
    knowledge_graphs (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        owner -> Text,
    }
}

diesel::table! {
    progress (user_id, graph) {
        user_id -> Text,
        graph -> Uuid,
        progress -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    resources (id) {
        title -> Text,
        description -> Text,
        topic_id -> Int8,
        link -> Nullable<Text>,
        id -> Int8,
    }
}

diesel::table! {
    topics (id) {
        knowledge_graph_id -> Uuid,
        knowledge_graph_index -> Int4,
        title -> Text,
        requirements -> Array<Nullable<Int4>>,
        id -> Int8,
        subject -> Text,
        content -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
    }
}

diesel::joinable!(progress -> knowledge_graphs (graph));
diesel::joinable!(progress -> users (user_id));
diesel::joinable!(resources -> topics (topic_id));
diesel::joinable!(topics -> knowledge_graphs (knowledge_graph_id));

diesel::allow_tables_to_appear_in_same_query!(
    knowledge_graphs,
    progress,
    resources,
    topics,
    users,
);
