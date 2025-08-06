use anyhow::{Context, Result};
use futures_util::StreamExt;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use tokio_util::io::StreamReader;

use crate::parser::{MslCommand, MslScript, MslValue};
use crate::scraper::{Scraper, ScrapingResult};

pub struct MslEngine {
    scraper: Scraper,
    variables: HashMap<String, String>,
    current_html: Option<String>,
    current_url: Option<String>,
}

impl MslEngine {
    pub fn new() -> Self {
        Self {
            scraper: Scraper::new(),
            variables: HashMap::new(),
            current_html: None,
            current_url: None,
        }
    }

    pub async fn execute(&mut self, script: MslScript) -> Result<()> {
        for command in script.commands {
            self.execute_command(command).await?;
        }
        Ok(())
    }

    async fn execute_command(&mut self, command: MslCommand) -> Result<()> {
        self.execute_command_sync(command).await
    }

    async fn execute_command_sync(&mut self, command: MslCommand) -> Result<()> {
        match command {
            MslCommand::Open { url } => {
                self.execute_open(url).await?;
            }
            MslCommand::Click { selector, commands } => {
                self.execute_click(selector, commands).await?;
            }
            MslCommand::Set { variable, value } => {
                self.execute_set(variable, value)?;
            }
            MslCommand::Media { media_blocks } => {
                self.execute_media(media_blocks).await?;
            }
            MslCommand::Save { path } => {
                self.execute_save(path).await?;
            }
            MslCommand::Wait { seconds } => {
                self.execute_wait(seconds).await?;
            }
        }
        Ok(())
    }

    async fn execute_open(&mut self, url: String) -> Result<()> {
        println!("Opening: {}", url);
        
        let result = self.scraper.fetch_page(&url).await?;
        // Store the HTML content for later use
        self.current_html = Some(self.get_html_content(&url).await?);
        self.current_url = Some(url);
        
        println!("Loaded page: {}", result.title.unwrap_or_else(|| "No title".to_string()));
        Ok(())
    }

    async fn execute_click(&mut self, selector: String, commands: Vec<MslCommand>) -> Result<()> {
        let html = self.current_html.as_ref()
            .context("No page loaded. Use 'open' first.")?;
        
        // Extract links matching the selector
        let links = self.scraper.extract_attribute(html, &selector, "href")?;
        
        if links.is_empty() {
            println!("No links found for selector: {}", selector);
            return Ok(());
        }

        // For now, follow the first link. In a more sophisticated version,
        // we could follow all links or implement pagination
        let link = &links[0];
        println!("Following link: {}", link);
        
        // Fetch the new page
        let result = self.scraper.fetch_page(link).await?;
        self.current_html = Some(self.get_html_content(link).await?);
        self.current_url = Some(link.clone());
        
        // Execute nested commands
        for command in commands {
            Box::pin(self.execute_command_sync(command)).await?;
        }
        
        Ok(())
    }

    fn execute_set(&mut self, variable: String, value: MslValue) -> Result<()> {
        let html = self.current_html.as_ref()
            .context("No page loaded. Use 'open' first.")?;
        
        let extracted_value = match value {
            MslValue::Text => {
                // For text extraction, we need a selector from the context
                // This is a simplified version - in practice, we'd need to track the current selector
                "".to_string() // Placeholder
            }
            MslValue::Attribute { name } => {
                // Similar to text, we need a selector
                "".to_string() // Placeholder
            }
            MslValue::Split { delimiter, index } => {
                // This would split a previously extracted value
                "".to_string() // Placeholder
            }
        };
        
        let variable_clone = variable.clone();
        let extracted_value_clone = extracted_value.clone();
        self.variables.insert(variable, extracted_value);
        println!("Set variable: {} = {}", variable_clone, extracted_value_clone);
        Ok(())
    }

    async fn execute_media(&mut self, media_blocks: Vec<crate::parser::MediaBlock>) -> Result<()> {
        let html = self.current_html.as_ref()
            .context("No page loaded. Use 'open' first.")?;
        
        let current_url = self.current_url.as_ref()
            .context("No current URL")?;
        
        // Extract all media from the current page
        let all_media = self.scraper.extract_media_from_html(html, current_url).await?;
        
        for block in media_blocks {
            let filtered_media = self.scraper.filter_media(&all_media, &block.filters);
            
            println!("Found {} {} items", filtered_media.len(), match block.media_type {
                crate::parser::MediaType::Image => "image",
                crate::parser::MediaType::Video => "video", 
                crate::parser::MediaType::Audio => "audio",
            });
            
            // For now, we'll use a default save path since the save command is separate
            // In a more sophisticated version, we'd track the save path from the save command
            let save_path = "./downloaded_media";
            
            // Download media items
            for media_item in filtered_media {
                self.download_media(&media_item, save_path).await?;
            }
        }
        
        Ok(())
    }

    async fn execute_save(&mut self, path: String) -> Result<()> {
        // This would save the current page or extracted data
        println!("Saving to: {}", path);
        Ok(())
    }

    async fn execute_wait(&mut self, seconds: u64) -> Result<()> {
        println!("Waiting for {} seconds...", seconds);
        tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
        println!("Wait completed.");
        Ok(())
    }

    async fn download_media(&self, media_item: &crate::scraper::MediaItem, base_path: &str) -> Result<()> {
        let url = &media_item.url;
        let filename = self.generate_filename(url, &media_item.media_type);
        
        // Create directory if it doesn't exist
        let dir = Path::new(base_path);
        if !dir.exists() {
            fs::create_dir_all(dir).await.context("Failed to create directory")?;
        }
        
        let file_path = dir.join(&filename);
        
        println!("Downloading: {} -> {}", url, file_path.display());
        
        // Download the file
        let response = self.scraper.client.get(url).send().await
            .context("Failed to download media")?;
        
        let mut file = fs::File::create(&file_path).await
            .context("Failed to create file")?;
        
        let bytes = response.bytes().await.context("Failed to read response bytes")?;
        tokio::io::copy(&mut std::io::Cursor::new(bytes), &mut file).await
            .context("Failed to write file")?;
        
        println!("Downloaded: {}", file_path.display());
        Ok(())
    }

    async fn get_html_content(&self, url: &str) -> Result<String> {
        self.scraper.get_html_content(url).await
    }

    fn generate_filename(&self, url: &str, media_type: &crate::scraper::MediaType) -> String {
        // Extract filename from URL or generate one
        let filename = url.split('/').last().unwrap_or("unknown");
        
        // Add appropriate extension if missing
        if !filename.contains('.') {
            let ext = match media_type {
                crate::scraper::MediaType::Image => "jpg",
                crate::scraper::MediaType::Video => "mp4",
                crate::scraper::MediaType::Audio => "mp3",
            };
            format!("{}.{}", filename, ext)
        } else {
            filename.to_string()
        }
    }
}

impl Default for MslEngine {
    fn default() -> Self {
        Self::new()
    }
} 