use anyhow::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingResult {
    pub url: String,
    pub title: Option<String>,
    pub links: Vec<String>,
    pub media: Vec<MediaItem>,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub url: String,
    pub media_type: MediaType,
    pub filename: Option<String>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaType {
    Image,
    Video,
    Audio,
}

pub struct Scraper {
    pub client: Client,
}

impl Scraper {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_page(&self, url: &str) -> Result<ScrapingResult> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to fetch page")?;

        let html = response.text().await.context("Failed to get response text")?;
        let document = Html::parse_document(&html);

        let mut result = ScrapingResult {
            url: url.to_string(),
            title: self.extract_title(&document),
            links: self.extract_links(&document, url)?,
            media: self.extract_media(&document, url)?,
            variables: HashMap::new(),
        };

        Ok(result)
    }

    pub fn extract_text(&self, html: &str, selector: &str) -> Result<Vec<String>> {
        let document = Html::parse_fragment(html);
        let selector = Selector::parse(selector).map_err(|e| anyhow::anyhow!("Invalid CSS selector: {}", e))?;

        let texts: Vec<String> = document
            .select(&selector)
            .map(|element| element.text().collect::<Vec<_>>().join(" "))
            .filter(|text| !text.trim().is_empty())
            .collect();

        Ok(texts)
    }

    pub fn extract_attribute(&self, html: &str, selector: &str, attribute: &str) -> Result<Vec<String>> {
        let document = Html::parse_fragment(html);
        let selector = Selector::parse(selector).map_err(|e| anyhow::anyhow!("Invalid CSS selector: {}", e))?;

        let attributes: Vec<String> = document
            .select(&selector)
            .filter_map(|element| element.value().attr(attribute).map(|s| s.to_string()))
            .collect();

        Ok(attributes)
    }

    fn extract_title(&self, document: &Html) -> Option<String> {
        document
            .select(&Selector::parse("title").unwrap())
            .next()
            .map(|title| title.text().collect::<Vec<_>>().join(" "))
    }

    fn extract_links(&self, document: &Html, base_url: &str) -> Result<Vec<String>> {
        let link_selector = Selector::parse("a[href]").unwrap();
        let mut links = Vec::new();

        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(base_url_parsed) = Url::parse(base_url) {
                    if let Ok(absolute_url) = base_url_parsed.join(href) {
                        links.push(absolute_url.to_string());
                    }
                }
            }
        }

        Ok(links)
    }

    fn extract_media(&self, document: &Html, base_url: &str) -> Result<Vec<MediaItem>> {
        let mut media_items = Vec::new();

        // Extract images
        let img_selector = Selector::parse("img[src]").unwrap();
        for element in document.select(&img_selector) {
            if let Some(src) = element.value().attr("src") {
                if let Ok(base_url_parsed) = Url::parse(base_url) {
                    if let Ok(absolute_url) = base_url_parsed.join(src) {
                        let mut attributes = HashMap::new();
                        for (key, value) in element.value().attrs() {
                            attributes.insert(key.to_string(), value.to_string());
                        }

                        media_items.push(MediaItem {
                            url: absolute_url.to_string(),
                            media_type: MediaType::Image,
                            filename: None,
                            attributes,
                        });
                    }
                }
            }
        }

        // Extract videos
        let video_selector = Selector::parse("video source[src], video[src]").unwrap();
        for element in document.select(&video_selector) {
            if let Some(src) = element.value().attr("src") {
                if let Ok(base_url_parsed) = Url::parse(base_url) {
                    if let Ok(absolute_url) = base_url_parsed.join(src) {
                        let mut attributes = HashMap::new();
                        for (key, value) in element.value().attrs() {
                            attributes.insert(key.to_string(), value.to_string());
                        }

                        media_items.push(MediaItem {
                            url: absolute_url.to_string(),
                            media_type: MediaType::Video,
                            filename: None,
                            attributes,
                        });
                    }
                }
            }
        }

        // Extract audio
        let audio_selector = Selector::parse("audio source[src], audio[src]").unwrap();
        for element in document.select(&audio_selector) {
            if let Some(src) = element.value().attr("src") {
                if let Ok(base_url_parsed) = Url::parse(base_url) {
                    if let Ok(absolute_url) = base_url_parsed.join(src) {
                        let mut attributes = HashMap::new();
                        for (key, value) in element.value().attrs() {
                            attributes.insert(key.to_string(), value.to_string());
                        }

                        media_items.push(MediaItem {
                            url: absolute_url.to_string(),
                            media_type: MediaType::Audio,
                            filename: None,
                            attributes,
                        });
                    }
                }
            }
        }

        Ok(media_items)
    }

    pub fn filter_media(&self, media: &[MediaItem], filters: &[crate::parser::MediaFilter]) -> Vec<MediaItem> {
        media
            .iter()
            .filter(|item| {
                filters.iter().all(|filter| match filter {
                    crate::parser::MediaFilter::Where { field, operator, value } => {
                        match field.as_str() {
                            "src" => {
                                let item_src = &item.url;
                                match operator.as_str() {
                                    "~" => item_src.contains(value),
                                    "=" => item_src == value,
                                    _ => true,
                                }
                            }
                            _ => true,
                        }
                    }
                    crate::parser::MediaFilter::Extensions { extensions } => {
                        let url = &item.url;
                        extensions.iter().any(|ext| url.ends_with(ext))
                    }
                })
            })
            .cloned()
            .collect()
    }

    pub async fn extract_media_from_html(&self, html: &str, base_url: &str) -> Result<Vec<MediaItem>> {
        let document = Html::parse_document(html);
        self.extract_media(&document, base_url)
    }

    pub async fn get_html_content(&self, url: &str) -> Result<String> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to fetch page")?;

        response.text().await.context("Failed to get response text")
    }
}

impl Default for Scraper {
    fn default() -> Self {
        Self::new()
    }
} 