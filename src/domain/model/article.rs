use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub thumbnail: String,
    pub image: String,
    pub description: String,
    pub body: String,
}

impl Article {
    pub fn builder() -> ArticleBuilder {
        ArticleBuilder::default()
    }
}

#[derive(Default)]
pub struct ArticleBuilder {
    id: Option<i32>,
    title: Option<String>,
    slug: Option<String>,
    thumbnail: Option<String>,
    image: Option<String>,
    description: Option<String>,
    body: Option<String>,
}

impl ArticleBuilder {
    pub fn new() -> ArticleBuilder {
        Self {
            id: None,
            title: None,
            slug: None,
            thumbnail: None,
            image: None,
            description: None,
            body: None,
        }
    }

    pub fn id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn slug(mut self, slug: String) -> Self {
        self.slug = Some(slug);
        self
    }

    pub fn thumbnail(mut self, thumbnail: String) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn image(mut self, image: String) -> Self {
        self.image = Some(image);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Article {
        Article {
            id: self.id.unwrap_or_default(),
            title: self.title.unwrap_or_default(),
            slug: self.slug.unwrap_or_default(),
            thumbnail: self.thumbnail.unwrap_or_default(),
            image: self.image.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            body: self.body.unwrap_or_default(),
        }
    }
}
