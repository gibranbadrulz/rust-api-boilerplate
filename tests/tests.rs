use rust_api_boilerplate::domain::model::article::Article;

#[test]
fn test_article_builder_default() {
    let article = Article::builder().build();

    assert_eq!(article.id, 0);
    assert_eq!(article.title, "");
    assert_eq!(article.slug, "");
    assert_eq!(article.thumbnail, "");
    assert_eq!(article.image, "");
    assert_eq!(article.description, "");
    assert_eq!(article.body, "");
}

#[test]
fn test_article_builder_with_values() {
    let article = Article::builder()
        .id(1)
        .title("Test Title".to_string())
        .slug("test-title".to_string())
        .thumbnail("https://cdn.example.com/test-image.jpg".to_string())
        .image("https://cdn.example.com/test-image.jpg".to_string())
        .description("Test description".to_string())
        .body("Test body".to_string())
        .build();

    assert_eq!(article.id, 1);
    assert_eq!(article.title, "Test Title");
    assert_eq!(article.slug, "test-title");
    assert_eq!(article.thumbnail, "https://cdn.example.com/test-image.jpg");
    assert_eq!(article.image, "https://cdn.example.com/test-image.jpg");
    assert_eq!(article.description, "Test description");
    assert_eq!(article.body, "Test body");
}

#[test]
fn test_article_builder_partial_values() {
    let article = Article::builder()
        .title("Partial Title".to_string())
        .build();

    assert_eq!(article.id, 0);
    assert_eq!(article.title, "Partial Title");
    assert_eq!(article.slug, "");
    assert_eq!(article.thumbnail, "");
    assert_eq!(article.image, "");
    assert_eq!(article.description, "");
    assert_eq!(article.body, "");
}

#[test]
fn test_article_builder_update_values() {
    let mut builder = Article::builder()
        .id(2)
        .title("Initial Title".to_string())
        .slug("initial-title".to_string());

    builder = builder
        .title("Updated Title".to_string())
        .image("https://cdn.example.com/test-image.jpg".to_string());

    let article = builder.build();

    assert_eq!(article.id, 2);
    assert_eq!(article.title, "Updated Title");
    assert_eq!(article.slug, "initial-title");
    assert_eq!(article.thumbnail, "");
    assert_eq!(article.image, "https://cdn.example.com/test-image.jpg");
    assert_eq!(article.description, "");
    assert_eq!(article.body, "");
}
