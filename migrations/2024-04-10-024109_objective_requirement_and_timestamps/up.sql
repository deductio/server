-- Make sure that when corresponding parents are deleted, the children are deleted too
-- (so, if a topic is deleted, all requirements involving the topic should be deleted as well).

ALTER TABLE requirements
    ADD FOREIGN KEY (knowledge_graph_id) REFERENCES knowledge_graphs (id) ON DELETE CASCADE,
    DROP CONSTRAINT fk_requirements_source_topics,
    DROP CONSTRAINT fk_requirements_destination_topics,
    ADD FOREIGN KEY (knowledge_graph_id, destination) REFERENCES topics (knowledge_graph_id, id) ON DELETE CASCADE,
    ADD FOREIGN KEY (knowledge_graph_id, source) REFERENCES topics (knowledge_graph_id, id) ON DELETE CASCADE;

ALTER TABLE objective_prerequisites
    ADD COLUMN topic_to_objective bool NOT NULL, -- TRUE if this is a topic satisfying an objective, FALSE if this is an objective satisfying a topic
    ADD FOREIGN KEY (knowledge_graph_id) REFERENCES knowledge_graphs (id) ON DELETE CASCADE,
    DROP CONSTRAINT fk_objective_prerequisites_topics,
    ADD FOREIGN KEY (topic) REFERENCES topics (id) ON DELETE CASCADE;

ALTER TABLE topics
    DROP CONSTRAINT fk_kg_topic,
    ADD FOREIGN KEY (knowledge_graph_id) REFERENCES knowledge_graphs (id) ON DELETE CASCADE;

CREATE UNIQUE INDEX op_single_topic ON objective_prerequisites (topic) WHERE topic_to_objective; -- only allow topics to satisfy a single objective

ALTER TABLE knowledge_graphs
    ADD COLUMN last_modified timestamp NOT NULL DEFAULT NOW();

CREATE OR REPLACE FUNCTION update_knowledge_graph_timestamp() RETURNS TRIGGER LANGUAGE plpgsql AS 
$$
DECLARE
    rec RECORD;
BEGIN
    FOR rec IN SELECT * FROM input_table LOOP
        UPDATE knowledge_graphs SET last_modified=NOW() WHERE id = rec.knowledge_graph_id;
    END LOOP;

    RETURN NULL;
END;
$$;

CREATE TRIGGER topics_update_timestamp_update
    AFTER UPDATE ON topics
    REFERENCING NEW TABLE AS input_table
    FOR EACH STATEMENT
    EXECUTE PROCEDURE update_knowledge_graph_timestamp();

CREATE TRIGGER topics_update_timestamp_insert
    AFTER INSERT ON topics
    REFERENCING NEW TABLE AS input_table
    FOR EACH STATEMENT
    EXECUTE PROCEDURE update_knowledge_graph_timestamp();

CREATE TRIGGER topics_update_timestamp_delete
    AFTER DELETE ON topics
    REFERENCING OLD TABLE AS input_table
    FOR EACH STATEMENT
    EXECUTE PROCEDURE update_knowledge_graph_timestamp();

CREATE TRIGGER requirements_update_timestamp_update
    AFTER UPDATE ON requirements
    REFERENCING NEW TABLE AS input_table
    FOR EACH STATEMENT
    EXECUTE PROCEDURE update_knowledge_graph_timestamp();

CREATE TRIGGER requirements_update_timestamp_insert
    AFTER INSERT ON requirements
    REFERENCING NEW TABLE AS input_table
    FOR EACH STATEMENT
    EXECUTE PROCEDURE update_knowledge_graph_timestamp();

CREATE TRIGGER requirements_update_timestamp_delete
    AFTER DELETE ON requirements
    REFERENCING OLD TABLE AS input_table
    FOR EACH STATEMENT
    EXECUTE PROCEDURE update_knowledge_graph_timestamp();