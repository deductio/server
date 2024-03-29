-- Your SQL goes here
ALTER TABLE requirements
    ADD COLUMN knowledge_graph_id uuid NOT NULL,
    ADD CONSTRAINT fk_requirements_knowledge_graphs FOREIGN KEY (knowledge_graph_id) REFERENCES knowledge_graphs (id),
    DROP CONSTRAINT IF EXISTS requirements_pkey,
    ADD COLUMN id BIGINT,
    ADD PRIMARY KEY (id);