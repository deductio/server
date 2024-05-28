DROP TABLE IF EXISTS learning_map_requirements;

CREATE TABLE learning_map_goals (
    learning_map_id BIGINT NOT NULL REFERENCES learning_maps (id) ON DELETE CASCADE,
    topic_id BIGINT NOT NULL REFERENCES topics (id) ON DELETE CASCADE,
    PRIMARY KEY (learning_map_id, topic_id)
);