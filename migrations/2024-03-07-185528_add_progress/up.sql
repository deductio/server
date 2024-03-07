CREATE TABLE users (
    id TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE progress (
    user_id TEXT NOT NULL,
    graph uuid NOT NULL,
    progress INT ARRAY NOT NULL,
    CONSTRAINT fk_knowledge_graphs_progress FOREIGN KEY (graph) REFERENCES knowledge_graphs (id),
    CONSTRAINT fk_user_progress FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (user_id, graph)
);