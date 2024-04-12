CREATE TABLE objectives (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    title TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL
);

CREATE TABLE objective_prerequisites (
    knowledge_graph_id uuid NOT NULL REFERENCES knowledge_graphs (id),
    topic BIGINT NOT NULL REFERENCES topics (id),
    objective BIGINT NOT NULL REFERENCES objectives (id),
    PRIMARY KEY (knowledge_graph_id, topic, objective)
);

CREATE INDEX requirements_knowledge_graph_id_idx ON requirements (knowledge_graph_id);

CREATE INDEX objective_prerequisites_knowledge_graph_id_idx ON objective_prerequisites (knowledge_graph_id);