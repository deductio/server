--- the database somehow doesn't already know this, let it know
ALTER TABLE topics
    ADD CONSTRAINT id_knowledge_graph_id_uq UNIQUE (knowledge_graph_id, id);

ALTER TABLE requirements
    DROP CONSTRAINT fk_destination_topics,
    DROP CONSTRAINT fk_source_topics,
    DROP CONSTRAINT fk_requirements_knowledge_graphs,
    ADD CONSTRAINT fk_requirements_source_topics FOREIGN KEY (knowledge_graph_id, source) REFERENCES topics (knowledge_graph_id, id),
    ADD CONSTRAINT fk_requirements_destination_topics FOREIGN KEY (knowledge_graph_id, destination) REFERENCES topics (knowledge_graph_id, id);

ALTER TABLE objective_prerequisites 
    DROP CONSTRAINT objective_prerequisites_knowledge_graph_id_fkey,
    DROP CONSTRAINT objective_prerequisites_topic_fkey,
    ADD CONSTRAINT fk_objective_prerequisites_topics FOREIGN KEY (knowledge_graph_id, topic) REFERENCES topics (knowledge_graph_id, id);