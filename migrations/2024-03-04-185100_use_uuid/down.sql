ALTER TABLE knowledge_graphs
    ALTER COLUMN id
    SET DATA TYPE BIGINT;

ALTER TABLE topics
    DROP CONSTRAINT fk_kg_topic

ALTER TABLE topics
    ALTER COLUMN knowledge_graph_id
    SET DATA TYPE BIGINT;

DROP INDEX topic_kg_idx;