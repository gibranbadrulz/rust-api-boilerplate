CREATE TABLE article_tags (
	id int4 GENERATED ALWAYS AS IDENTITY( INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1 NO CYCLE) NOT NULL,
	article_id int4 NOT NULL,
	tag_id int4 NOT NULL,
	created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
	updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
	CONSTRAINT article_tags_pkey PRIMARY KEY (id),
	CONSTRAINT fk_article FOREIGN KEY (article_id) REFERENCES "article"(id) ON DELETE CASCADE,
	CONSTRAINT fk_tags FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE RESTRICT
);

CREATE INDEX idx_article_tags_tag_id ON public.article_tags USING btree (tag_id);
