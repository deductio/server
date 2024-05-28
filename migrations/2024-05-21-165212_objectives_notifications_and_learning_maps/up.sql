
--- SQL related to objectives ---
ALTER TABLE objectives
    ADD COLUMN tsv_title_desc tsvector 
    GENERATED ALWAYS AS (to_tsvector('english', coalesce(title, '') || ' ' || coalesce(description, ''))) 
    STORED NOT NULL;

CREATE INDEX objective_tsv_title_desc_idx ON objectives USING GIN (tsv_title_desc);

ALTER TABLE objective_prerequisites 
    DROP CONSTRAINT IF EXISTS op_single_topic,
    DROP COLUMN IF EXISTS topic_to_objective,
    ADD COLUMN IF NOT EXISTS suggested_topic BIGINT NOT NULL,
    ADD COLUMN IF NOT EXISTS suggested_graph UUID NOT NULL,
    ADD FOREIGN KEY (suggested_topic, suggested_graph) REFERENCES topics (id, knowledge_graph_id) ON DELETE SET NULL;

CREATE TABLE objective_satisfiers (
    knowledge_graph_id UUID NOT NULL,
    objective BIGINT NOT NULL REFERENCES objectives (id) ON DELETE CASCADE,
    topic BIGINT NOT NULL,
    FOREIGN KEY (knowledge_graph_id, topic) REFERENCES topics (knowledge_graph_id, id) ON DELETE CASCADE,
    PRIMARY KEY (knowledge_graph_id, topic)
);

--- SQL related to notifications ---
ALTER TABLE likes ALTER COLUMN like_date SET DEFAULT NOW();

CREATE TABLE notifications (
    user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    notif_time TIMESTAMP NOT NULL DEFAULT NOW(),
    content JSONB NOT NULL,
    dismissal_time TIMESTAMP,
    PRIMARY KEY (user_id, content)
);

CREATE OR REPLACE FUNCTION send_like_notification() RETURNS TRIGGER LANGUAGE plpgsql AS 
$$
DECLARE
    graph_id uuid;
    graph_name TEXT;
    graph_author BIGINT;
    author_id BIGINT;
    username TEXT; 
BEGIN
    SELECT id, name, author 
        FROM knowledge_graphs 
        INTO graph_id, graph_name, graph_author 
        WHERE knowledge_graphs.id = NEW.knowledge_graph_id;

    author_id = (SELECT id FROM users WHERE users.id = graph_author);
    username := (SELECT users.username FROM users WHERE users.id = NEW.user_id);
    
    INSERT INTO notifications (user_id, content) VALUES (
        author_id,
        jsonb_build_object('type', 'like', 'user', username, 'graph_id', graph_id, 'graph_name', graph_name)
    ) ON CONFLICT (user_id, content) DO UPDATE SET notif_time = NOW(), dismissal_time = NULL;
    RETURN NULL;
END;
$$;

CREATE OR REPLACE FUNCTION remove_like_notification() RETURNS TRIGGER LANGUAGE plpgsql AS
$$
BEGIN
    UPDATE notifications SET dismissal_time = NOW() WHERE 
        notifications.content->>'type' = 'like' 
        AND (notifications.content->>'graph_id')::uuid = OLD.knowledge_graph_id;
    RETURN NULL;
END;
$$;

CREATE TRIGGER sync_like_notif_insert AFTER INSERT ON likes FOR EACH ROW EXECUTE FUNCTION send_like_notification();

CREATE TRIGGER sync_like_notif_delete AFTER DELETE ON likes FOR EACH ROW EXECUTE FUNCTION remove_like_notification();

CREATE OR REPLACE FUNCTION notify_learning_map_failure() RETURNS TRIGGER LANGUAGE plpgsql AS
$$
DECLARE
    graph_id uuid;
    graph_name TEXT;
    graph_author BIGINT;
BEGIN   
    SELECT id, name, author 
        FROM knowledge_graphs 
        INTO graph_id, graph_name, graph_author 
        WHERE knowledge_graphs.id = OLD.knowledge_graph_id;

    INSERT INTO notifications (user_id, content) VALUES (
        graph_author,
        jsonb_build_object('type', 'learning_map_failure', 'graph_id', graph_id, 'graph_name', graph_name)
    ) ON CONFLICT (user_id, content) DO NOTHING;
END;
$$;

CREATE TRIGGER catch_satisfier_deletion AFTER DELETE ON objective_satisfiers FOR EACH ROW EXECUTE FUNCTION notify_learning_map_failure();

--- SQL related to learning maps ---
CREATE TABLE learning_maps (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    title TEXT NOT NULL
);

CREATE TABLE learning_map_components (
    learning_map_id BIGINT NOT NULL REFERENCES learning_maps (id) ON DELETE CASCADE,
    source uuid NOT NULL REFERENCES knowledge_graphs (id) ON DELETE SET NULL,
    destination uuid NOT NULL REFERENCES knowledge_graphs (id) ON DELETE SET NULL,
    objective BIGINT NOT NULL REFERENCES objectives (id) ON DELETE CASCADE,
    PRIMARY KEY (learning_map_id, objective)
);

CREATE OR REPLACE FUNCTION avoid_learning_map_cycles() RETURNS TRIGGER
LANGUAGE plpgsql
AS 
$$
BEGIN
    	IF EXISTS (
          WITH RECURSIVE temp(source, destination) AS (
            SELECT learning_map_components.source, learning_map_components.destination
            FROM extensions
            WHERE learning_map_components.source = NEW.source AND learning_map_components.destination = NEW.destination
          UNION ALL
            SELECT learning_map_components.source, learning_map_components.destination
            FROM learning_map_components, temp
            WHERE learning_map_components.source = temp.destination

        	) CYCLE source SET is_cycle USING path
        	SELECT is_cycle FROM temp WHERE is_cycle=true LIMIT 1
        ) THEN
        	RAISE 'Cycle detected';
        ELSE 
        	RETURN NULL;
        END IF;
 END;
$$;

CREATE TRIGGER avoid_learning_map_cycles_trigger 
    AFTER INSERT ON learning_map_components 
    FOR EACH ROW 
    EXECUTE FUNCTION avoid_learning_map_cycles();