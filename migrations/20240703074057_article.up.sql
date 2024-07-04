CREATE TABLE article (
    id int4 GENERATED ALWAYS AS IDENTITY( INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1 NO CYCLE) NOT NULL,
    title varchar(255) NOT NULL,
    slug varchar(255) NOT NULL,
    thumbnail varchar(255) NOT NULL,
    image varchar(255) NOT NULL,
    description varchar(255) NOT NULL,
    body text NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    author_id int4 NOT NULL,
    is_active bool DEFAULT true NOT NULL,
    CONSTRAINT article_pkey PRIMARY KEY (id)
);
