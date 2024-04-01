ALTER TABLE knowledge_graphs
    DROP COLUMN owner;

ALTER TABLE progress
    DROP CONSTRAINT fk_user_progress,
    DROP COLUMN user_id;

ALTER TABLE users
    ADD COLUMN github_user_id TEXT UNIQUE,
    ADD COLUMN google_user_id TEXT UNIQUE,
    ADD COLUMN username TEXT NOT NULL UNIQUE,
    ADD COLUMN avatar TEXT,
    DROP CONSTRAINT users_pkey,
    DROP COLUMN id,
    ADD COLUMN id BIGSERIAL,
    ADD PRIMARY KEY (id),

    -- Only allow users to have one sign-in method
    ADD CONSTRAINT chk_one_sso CHECK (num_nonnulls(google_user_id, github_user_id) = 1),
    ADD CONSTRAINT chk_username CHECK (
        LENGTH(username) < 20 AND 
        LENGTH(username) > 3 AND
        username ~* '^[a-z0-9._]+$');

ALTER TABLE knowledge_graphs
    ADD COLUMN author BIGINT NOT NULL,
    ADD CONSTRAINT fk_knowledge_graphs_user FOREIGN KEY (author) REFERENCES users (id),
    ADD CONSTRAINT chk_author_title_unique UNIQUE (author, name);

ALTER TABLE progress
    ADD COLUMN user_id BIGINT NOT NULL,
    ADD CONSTRAINT fk_progress_users FOREIGN KEY (user_id) REFERENCES users (id),
    ADD PRIMARY KEY (user_id, graph);