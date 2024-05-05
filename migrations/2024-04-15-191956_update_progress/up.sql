ALTER TABLE progress
    DROP COLUMN user_progress,
    DROP COLUMN graph,
    ADD COLUMN knowledge_graph_id uuid NOT NULL,
    DROP CONSTRAINT IF EXISTS progress_pkey,
    ADD COLUMN topic BIGINT NOT NULL,
    ADD CONSTRAINT progress_kgi_topic FOREIGN KEY (knowledge_graph_id, topic) REFERENCES topics (knowledge_graph_id, id),
    ADD PRIMARY KEY (user_id, topic);

CREATE INDEX progress_knowledge_graph_id_idx ON progress (user_id, knowledge_graph_id);