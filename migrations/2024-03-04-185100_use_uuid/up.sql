ALTER TABLE knowledge_graphs 
    ALTER COLUMN id DROP DEFAULT,
    ALTER COLUMN id TYPE uuid USING (gen_random_uuid());

ALTER TABLE topics
    ALTER COLUMN knowledge_graph_id DROP DEFAULT,
    ALTER COLUMN knowledge_graph_id TYPE uuid USING (gen_random_uuid());

ALTER TABLE topics
    ADD CONSTRAINT fk_kg_topic FOREIGN KEY (knowledge_graph_id) 
    REFERENCES knowledge_graphs (id);

ALTER TABLE resources
    ADD CONSTRAINT fk_topic_resource FOREIGN KEY (topic_id)
    REFERENCES topics (id);

CREATE INDEX topic_kg_idx ON topics (knowledge_graph_id);