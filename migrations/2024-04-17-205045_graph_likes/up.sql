ALTER TABLE knowledge_graphs
  ADD COLUMN IF NOT EXISTS like_count INT NOT NULL DEFAULT 0,
  ALTER COLUMN last_modified SET DATA TYPE DATE;

CREATE TABLE likes (
  knowledge_graph_id uuid REFERENCES knowledge_graphs (id) NOT NULL,
  user_id BIGINT references users (id) NOT NULL,
  like_date date NOT NULL,
  PRIMARY KEY (knowledge_graph_id, user_id)
);

CREATE OR REPLACE FUNCTION increment_like_count() RETURNS TRIGGER LANGUAGE plpgsql AS 
$$
BEGIN
  UPDATE knowledge_graphs SET like_count = like_count + 1 WHERE id = NEW.knowledge_graph_id;
END;
$$;

CREATE OR REPLACE FUNCTION decrement_like_count() RETURNS TRIGGER LANGUAGE plpgsql AS 
$$
BEGIN
  UPDATE knowledge_graphs SET like_count = like_count - 1 WHERE id = OLD.knowledge_graph_id;
END;
$$;

CREATE TRIGGER sync_likes_increment AFTER INSERT ON likes FOR EACH ROW EXECUTE FUNCTION increment_like_count();

CREATE TRIGGER sync_likes_decrement AFTER DELETE ON likes FOR EACH ROW EXECUTE FUNCTION decrement_like_count();

CREATE INDEX likes_knowledge_graph_id_idx ON likes (knowledge_graph_id);

CREATE INDEX likes_user_id_idx ON likes (user_id);