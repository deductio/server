-- This file should undo anything in `up.sql`
DROP INDEX progress_knowledge_graph_id_idx;

ALTER TABLE progress    
    DROP CONSTRAINT progress_pkey,
    DROP COLUMN knowledge_graph_id,
    DROP COLUMN topic,
    ADD COLUMN user_progress BIGINT[] NOT NULL,
    ADD COLUMN graph uuid NOT NULL REFERENCES knowledge_graphs (id);