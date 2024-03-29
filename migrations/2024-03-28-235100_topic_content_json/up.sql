ALTER TABLE topics 
    ALTER COLUMN content SET DATA TYPE jsonb USING ('{}'::json),
    DROP COLUMN knowledge_graph_index;