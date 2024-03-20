ALTER TABLE topics
	DROP CONSTRAINT IF EXISTS topics_requirements_check,
	DROP COLUMN IF EXISTS requirements;

CREATE TABLE IF NOT EXISTS extensions (
  source uuid NOT NULL,
  destination uuid NOT NULL,
  CONSTRAINT fk_source_knowledge_graph FOREIGN KEY (source) REFERENCES knowledge_graphs (id),
  CONSTRAINT fk_destination_knowledge_graph FOREIGN KEY (destination) REFERENCES knowledge_graphs (id),
  PRIMARY KEY (source, destination)
);

CREATE OR REPLACE FUNCTION avoid_extension_cycles() RETURNS TRIGGER
LANGUAGE plpgsql
AS 
$$
BEGIN
    	IF EXISTS (
          WITH RECURSIVE temp(source, destination) AS (
            SELECT extensions.source, extensions.destination
            FROM extensions
            WHERE extensions.source = NEW.source AND extensions.destination = NEW.destination
          UNION ALL
            SELECT extensions.source, extensions.destination
            FROM extensions, temp
            WHERE extensions.source = temp.destination

        	) CYCLE source SET is_cycle USING path
        	SELECT is_cycle FROM temp WHERE is_cycle=true LIMIT 1
        ) THEN
        	RAISE 'Cycle detected';
        ELSE 
        	RETURN NULL;
        END IF;
 END;
$$;

CREATE TRIGGER avoid_cycles_extensions_trigger AFTER INSERT ON extensions FOR EACH ROW EXECUTE FUNCTION avoid_extension_cycles();

CREATE TABLE IF NOT EXISTS requirements (
  source bigint NOT NULL,
  destination bigint NOT NULL,
  CONSTRAINT fk_source_topics FOREIGN KEY (source) REFERENCES topics (id),
  CONSTRAINT fk_destination_topics FOREIGN KEY (destination) REFERENCES topics (id),
  PRIMARY KEY (source, destination)
);

CREATE OR REPLACE FUNCTION avoid_requirement_cycles() RETURNS TRIGGER
LANGUAGE plpgsql
AS 
$$
BEGIN
    	IF EXISTS (
          WITH RECURSIVE temp(source, destination) AS (
            SELECT requirements.source, requirements.destination
            FROM requirements
            WHERE requirements.source = NEW.source AND requirements.destination = NEW.destination
          UNION ALL
            SELECT requirements.source, requirements.destination
            FROM requirements, temp
            WHERE requirements.source = temp.destination

        	) CYCLE source SET is_cycle USING path
        	SELECT is_cycle FROM temp WHERE is_cycle=true LIMIT 1
        ) THEN
        	RAISE 'Cycle detected';
        ELSE 
        	RETURN NULL;
        END IF;
 END;
$$;

CREATE TRIGGER avoid_cycles_requirements_trigger AFTER INSERT ON requirements FOR EACH ROW EXECUTE FUNCTION avoid_requirement_cycles();

ALTER TABLE progress
  RENAME COLUMN progress to user_progress;