ALTER TABLE knowledge_graphs
    ADD COLUMN tsv_name_desc tsvector GENERATED ALWAYS AS (to_tsvector('english', coalesce(name, '') || ' ' || coalesce(description, ''))) STORED NOT NULL;

CREATE INDEX knowledge_graph_tsv_name_desc_idx ON knowledge_graphs USING GIN (tsv_name_desc);