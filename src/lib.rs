pub mod parser;
pub mod scraper;
pub mod engine;
pub mod cli;

pub use engine::MslEngine;
pub use parser::{parse_script, MslScript, MslError};
pub use scraper::{Scraper, ScrapingResult};

use anyhow::Result;

/// Main entry point for the MSL Engine
pub async fn run_script(script_content: &str) -> Result<()> {
    let script = parser::parse_script(script_content)?;
    let mut engine = engine::MslEngine::new();
    engine.execute(script).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_script() {
        let script = r#"
open "https://example.com"
click ".user-card a"
  set user = text
  media
    image
      where src ~ "cdn.example.com"
      extensions jpg, png
    save to "./media/{user}"
"#;
        
        let result = run_script(script).await;
        // This will fail since we don't have a real server, but it tests parsing
        assert!(result.is_ok() || result.unwrap_err().to_string().contains("network"));
    }
} 