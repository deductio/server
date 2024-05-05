ALTER TABLE knowledge_graphs
    DROP COLUMN IF EXISTS like_count,
    ALTER COLUMN last_modified SET DATA TYPE TIMESTAMP;

DROP TRIGGER IF EXISTS sync_likes_decrement ON likes;

DROP TRIGGER IF EXISTS sync_likes_decrement ON likes;

DROP TABLE IF EXISTS likes;

DROP FUNCTION IF EXISTS increment_like_count;

DROP FUNCTION IF EXISTS decrement_like_count;

