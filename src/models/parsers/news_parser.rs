use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct NewsResponse {
    pub status: String,
    #[serde(rename = "totalResults")]
    pub total_results: i32,
    pub articles: Vec<Article>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct Article {
    #[serde(default)]
    pub source: Option<Source>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "urlToImage", default)]
    pub url_to_image: Option<String>,
    #[serde(rename = "publishedAt", default)]
    pub published_at: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct Source {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}
