-- This file should undo anything in `up.sql`
ALTER TABLE topics
    ALTER COLUMN content SET DATA TYPE TEXT USING (content::text);