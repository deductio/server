-- Your SQL goes here
ALTER TABLE knowledge_graphs 
    ALTER COLUMN id SET DEFAULT gen_random_uuid();