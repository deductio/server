ALTER TABLE topics
    ADD COLUMN original_topic BIGINT,
    CONSTRAINT fk_topics_otopics FOREIGN KEY (original_topic) REFERENCES topics (id); 

-- underlying graph can be deduced from original topic id, no need for separate graph



