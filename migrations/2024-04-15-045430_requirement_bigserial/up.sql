CREATE SEQUENCE requirements_id_seq;

ALTER TABLE requirements
    ALTER COLUMN id SET DEFAULT nextval('requirements_id_seq');

ALTER SEQUENCE requirements_id_seq OWNED BY requirements.id;