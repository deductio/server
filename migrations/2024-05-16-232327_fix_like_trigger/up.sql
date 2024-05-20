DROP TRIGGER sync_likes_increment ON likes;

DROP TRIGGER sync_likes_decrement ON likes;

CREATE OR REPLACE FUNCTION increment_like_count() RETURNS TRIGGER LANGUAGE plpgsql AS 
$$
BEGIN
  UPDATE knowledge_graphs SET like_count = like_count + 1 WHERE id = NEW.knowledge_graph_id;
  RETURN NULL;
END;
$$;

CREATE OR REPLACE FUNCTION decrement_like_count() RETURNS TRIGGER LANGUAGE plpgsql AS 
$$
BEGIN
  UPDATE knowledge_graphs SET like_count = like_count - 1 WHERE id = OLD.knowledge_graph_id;
  RETURN NULL;
END;
$$;

CREATE TRIGGER sync_likes_increment AFTER INSERT ON likes FOR EACH ROW EXECUTE FUNCTION increment_like_count();

CREATE TRIGGER sync_likes_decrement AFTER DELETE ON likes FOR EACH ROW EXECUTE FUNCTION decrement_like_count();