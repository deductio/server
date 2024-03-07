ALTER TABLE resources 
    DROP CONSTRAINT fk_topic_resource;

-- need to do this first, sort of cyclic madness

ALTER TABLE topics
    ALTER COLUMN requirements SET NOT NULL,
    ADD CONSTRAINT topics_requirements_check 
        CHECK (array_position(requirements, NULL) IS NULL),
    DROP CONSTRAINT topics_pkey,
    DROP COLUMN id,
    ADD PRIMARY KEY (knowledge_graph_id, knowledge_graph_index);

ALTER TABLE resources 
    DROP CONSTRAINT resources_pkey,
    DROP COLUMN id,
    ADD COLUMN knowledge_graph_id uuid NOT NULL DEFAULT gen_random_uuid(),
    ADD COLUMN resource_offset INT NOT NULL DEFAULT 1,
    ADD PRIMARY KEY (knowledge_graph_id, topic_id, resource_offset),
    ADD CONSTRAINT fk_topic_resource FOREIGN KEY (knowledge_graph_id, topic_id) 
        REFERENCES topics (knowledge_graph_id, knowledge_graph_index);
