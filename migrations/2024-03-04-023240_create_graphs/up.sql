CREATE TABLE IF NOT EXISTS knowledge_graphs (
    id             BIGSERIAL NOT NULL PRIMARY KEY,
    name           TEXT NOT NULL,
    description    TEXT NOT NULL,
    owner          TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS topics (
    id                       BIGSERIAL NOT NULL PRIMARY KEY,
    knowledge_graph_id       BIGINT NOT NULL,
    knowledge_graph_index    INT NOT NULL,
    title                    TEXT NOT NULL,
    requirements             INT ARRAY
);

CREATE TABLE IF NOT EXISTS resources (
    id             BIGSERIAL NOT NULL PRIMARY KEY,
    title          TEXT NOT NULL,
    description    TEXT NOT NULL,
    topic_id       BIGINT NOT NULL,
    link           TEXT
);