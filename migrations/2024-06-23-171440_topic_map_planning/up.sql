ALTER TABLE topics
    DROP COLUMN subject,
    ADD COLUMN description TEXT NOT NULL DEFAULT '';

ALTER TABLE user_objective_progress ADD COLUMN objective_time TIMESTAMP NOT NULL DEFAULT NOW();

ALTER TABLE learning_maps ADD COLUMN creation_time TIMESTAMP NOT NULL DEFAULT NOW();