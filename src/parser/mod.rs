use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{char, multispace0, multispace1, none_of},
    combinator::{map, opt, value},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MslError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MslScript {
    pub commands: Vec<MslCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MslCommand {
    Open { url: String },
    Click { selector: String, commands: Vec<MslCommand> },
    Set { variable: String, value: MslValue },
    Media { media_blocks: Vec<MediaBlock> },
    Save { path: String },
    Wait { seconds: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MslValue {
    Text,
    Attribute { name: String },
    Split { delimiter: String, index: i32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaBlock {
    pub media_type: MediaType,
    pub filters: Vec<MediaFilter>,
    pub save_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaType {
    Image,
    Video,
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaFilter {
    Where { field: String, operator: String, value: String },
    Extensions { extensions: Vec<String> },
}

pub fn parse_script(input: &str) -> Result<MslScript, MslError> {
    let (remaining, commands) = many0(parse_command)(input)
        .map_err(|e| MslError::ParseError(format!("Failed to parse script: {}", e)))?;
    
    if !remaining.trim().is_empty() {
        return Err(MslError::ParseError(format!("Unparsed content: {}", remaining)));
    }
    
    Ok(MslScript { commands })
}

fn parse_command(input: &str) -> IResult<&str, MslCommand> {
    let (input, _) = multispace0(input)?;
    alt((
        parse_open,
        parse_click,
        parse_set,
        parse_media,
        parse_save,
        parse_wait,
    ))(input)
}

fn parse_open(input: &str) -> IResult<&str, MslCommand> {
    let (input, _) = tag("open")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, url) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    let (input, _) = multispace0(input)?;
    
    Ok((input, MslCommand::Open { url: url.to_string() }))
}

fn parse_click(input: &str) -> IResult<&str, MslCommand> {
    let (input, _) = tag("click")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, selector) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = opt(char('\n'))(input)?;
    
    // Parse indented commands
    let (input, commands) = many0(parse_indented_command)(input)?;
    
    Ok((input, MslCommand::Click { 
        selector: selector.to_string(), 
        commands 
    }))
}

fn parse_indented_command(input: &str) -> IResult<&str, MslCommand> {
    let (input, _) = multispace1(input)?;
    let (input, _) = multispace1(input)?; // Double indent
    parse_command(input)
}

fn parse_set(input: &str) -> IResult<&str, MslCommand> {
    let (input, _) = tag("set")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, variable) = take_while(|c| c != ' ' && c != '=')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = parse_value(input)?;
    
    Ok((input, MslCommand::Set { 
        variable: variable.to_string(), 
        value 
    }))
}

fn parse_value(input: &str) -> IResult<&str, MslValue> {
    alt((
        parse_text_value,
        parse_attribute_value,
        parse_split_value,
    ))(input)
}

fn parse_text_value(input: &str) -> IResult<&str, MslValue> {
    let (input, _) = tag("text")(input)?;
    Ok((input, MslValue::Text))
}

fn parse_attribute_value(input: &str) -> IResult<&str, MslValue> {
    let (input, _) = tag("attr")(input)?;
    let (input, _) = delimited(char('('), char('"'), char(')'))(input)?;
    let (input, attr_name) = take_until("\"")("(\"")?;
    let (input, _) = char('"')(input)?;
    let (input, _) = char(')')(input)?;
    
    Ok((input, MslValue::Attribute { 
        name: attr_name.to_string() 
    }))
}

fn parse_split_value(input: &str) -> IResult<&str, MslValue> {
    let (input, _) = tag("split")(input)?;
    let (input, _) = delimited(char('('), char('"'), char(')'))(input)?;
    let (input, delimiter) = take_until("\"")("(\"")?;
    let (input, _) = char('"')(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = char('.')(input)?;
    let (input, _) = tag("split")(input)?;
    let (input, _) = char('(')(input)?;
    let (input, _) = char('"')(input)?;
    let (input, split_delimiter) = take_until("\"")("(\"")?;
    let (input, _) = char('"')(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = char('[')(input)?;
    let (input, index_str) = take_while(|c| c != ']')(input)?;
    let (input, _) = char(']')(input)?;
    
    let index = index_str.parse::<i32>().unwrap_or(-1);
    
    Ok((input, MslValue::Split { 
        delimiter: split_delimiter.to_string(), 
        index 
    }))
}

fn parse_media(input: &str) -> IResult<&str, MslCommand> {
    let (input, _) = tag("media")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = opt(char('\n'))(input)?;
    
    let (input, media_blocks) = many0(parse_media_block)(input)?;
    
    let (input, _) = opt(char('\n'))(input)?;
    
    Ok((input, MslCommand::Media { media_blocks }))
}

fn parse_media_block(input: &str) -> IResult<&str, MediaBlock> {
    let (input, _) = multispace0(input)?;
    let (input, media_type) = parse_media_type(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = opt(char('\n'))(input)?;
    
    let (input, filters) = many0(parse_media_filter)(input)?;
    
    let (input, _) = opt(char('\n'))(input)?;
    
    Ok((input, MediaBlock { 
        media_type, 
        filters, 
        save_path: None 
    }))
}

fn parse_media_type(input: &str) -> IResult<&str, MediaType> {
    alt((
        value(MediaType::Image, tag("image")),
        value(MediaType::Video, tag("video")),
        value(MediaType::Audio, tag("audio")),
    ))(input)
}

fn parse_media_filter(input: &str) -> IResult<&str, MediaFilter> {
    let (input, _) = multispace0(input)?;
    
    let (input, filter) = alt((
        parse_where_filter,
        parse_extensions_filter,
    ))(input)?;
    
    let (input, _) = opt(char('\n'))(input)?;
    
    Ok((input, filter))
}

fn parse_where_filter(input: &str) -> IResult<&str, MediaFilter> {
    let (input, _) = tag("where")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, field) = take_while(|c| c != ' ')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, operator) = alt((tag("~"), tag("="), tag("!=")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    let (input, _) = opt(char('\n'))(input)?;
    
    Ok((input, MediaFilter::Where { 
        field: field.to_string(), 
        operator: operator.to_string(), 
        value: value.to_string() 
    }))
}

fn parse_extensions_filter(input: &str) -> IResult<&str, MediaFilter> {
    let (input, _) = tag("extensions")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, extensions) = separated_list0(
        preceded(opt(multispace0), char(',')),
        preceded(opt(multispace0), take_while(|c| c != ' ' && c != '\n' && c != ','))
    )(input)?;
    
    let extensions_vec = extensions.iter().map(|s| s.to_string()).collect();
    
    let (input, _) = opt(char('\n'))(input)?;
    
    Ok((input, MediaFilter::Extensions { extensions: extensions_vec }))
}

fn parse_save_path(input: &str) -> IResult<&str, String> {
    let (input, _) = multispace1(input)?;
    let (input, _) = multispace1(input)?; // Double indent
    let (input, _) = tag("save")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("to")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, path) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    let (input, _) = opt(char('\n'))(input)?;
    
    Ok((input, path.to_string()))
}

fn parse_save(input: &str) -> IResult<&str, MslCommand> {
    let (input, _) = tag("save")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("to")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, path) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    
    Ok((input, MslCommand::Save { path: path.to_string() }))
}

fn parse_wait(input: &str) -> IResult<&str, MslCommand> {
    let (input, _) = tag("wait")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, seconds_str) = take_while(|c: char| c.is_ascii_digit())(input)?;
    let (input, _) = multispace0(input)?;
    
    let seconds = seconds_str.parse::<u64>().unwrap_or(1);
    
    Ok((input, MslCommand::Wait { seconds }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_open() {
        let input = r#"open "https://example.com""#;
        let result = parse_open(input);
        assert!(result.is_ok());
        if let Ok((_, MslCommand::Open { url })) = result {
            assert_eq!(url, "https://example.com");
        }
    }

    #[test]
    fn test_parse_click() {
        let input = r#"click ".user-card a"
  set user = text"#;
        let result = parse_click(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_script() {
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
        let result = parse_script(script);
        assert!(result.is_ok());
    }
} 