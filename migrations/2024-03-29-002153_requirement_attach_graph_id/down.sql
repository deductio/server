-- This file should undo anything in `up.sql`
ALTER TABLE requirements
    DROP COLUMN IF EXISTS graph,
    DROP CONSTRAINT IF EXISTS fk_requirements_knowledge_graphs,
    DROP COLUMN IF EXISTS knowledge_graph_id;