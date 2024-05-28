DROP TRIGGER avoid_learning_map_cycles_trigger ON learning_map_components;

DROP FUNCTION avoid_learning_map_cycles;

DROP TABLE learning_map_components;

CREATE TABLE learning_map_requirements (
    source BIGINT NOT NULL REFERENCES topics (id) ON DELETE CASCADE,
    destination BIGINT NOT NULL REFERENCES topics (id) ON DELETE CASCADE,
    learning_map_id BIGINT NOT NULL REFERENCES learning_maps (id) ON DELETE CASCADE,
    PRIMARY KEY (learning_map_id, source, destination)
);

CREATE INDEX learning_map_requirement_map_id_idx ON learning_map_requirements (learning_map_id);