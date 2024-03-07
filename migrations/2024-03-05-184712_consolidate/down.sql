ALTER TABLE topics
    DROP CONSTRAINT topic_pkey,
    ADD COLUMN id BIGSERIAL NOT NULL,
    ADD PRIMARY KEY (id);

ALTER TABLE resources 
    DROP CONSTRAINT resources_pkey,
    ADD COLUMN id BIGSERIAL NOT NULL,
    DROP COLUMN knowledge_graph_id,
    DROP CONSTRAINT fk_topic_resource,
    ADD CONSTRAINT fk_topic_resource FOREIGN KEY (topic_id)
        REFERENCES topics (id),
    ADD PRIMARY KEY (id);
    

