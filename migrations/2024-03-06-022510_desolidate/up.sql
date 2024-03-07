ALTER TABLE topics
    ADD COLUMN id BIGSERIAL NOT NULL;

ALTER TABLE resources 
    DROP CONSTRAINT resources_pkey,
    ADD COLUMN id BIGSERIAL NOT NULL,
    DROP COLUMN knowledge_graph_id,
    ADD PRIMARY KEY (id);

ALTER TABLE topics
    DROP CONSTRAINT topics_pkey,
    ADD PRIMARY KEY (id);
    
ALTER TABLE resources
    ADD CONSTRAINT fk_topic_resource FOREIGN KEY (topic_id)
        REFERENCES topics (id);