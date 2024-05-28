CREATE TABLE user_objective_progress (
    user_id BIGINT REFERENCES users (id) ON DELETE CASCADE,
    objective_id BIGINT REFERENCES objectives (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, objective_id)
);

CREATE OR REPLACE FUNCTION update_objective_progress() RETURNS TRIGGER LANGUAGE plpgsql AS $$
DECLARE
    objective BIGINT;
BEGIN
    objective := (SELECT id FROM objective_satisfiers os WHERE NEW.topic = os.topic);

    IF objective IS NOT NULL THEN
        INSERT INTO user_objective_progress VALUES (NEW.user_id, objective);
    END IF;
END;
$$;