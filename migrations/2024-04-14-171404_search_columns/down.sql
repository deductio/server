DROP INDEX knowledge_graph_tsv_name_desc_idx;

ALTER TABLE knowledge_graphs
    DROP COLUMN tsv_name_desc;