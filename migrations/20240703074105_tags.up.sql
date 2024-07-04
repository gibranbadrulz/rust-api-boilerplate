CREATE TABLE tags (
    id int4 GENERATED ALWAYS AS IDENTITY( INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1 NO CYCLE) NOT NULL,
    "name" varchar(50) NOT NULL,
    slug varchar(50) NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    is_active bool DEFAULT true NOT NULL,
    CONSTRAINT tags_pkey PRIMARY KEY (id)
);