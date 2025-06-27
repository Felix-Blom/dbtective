use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SqlResourceInfo {
    pub name: String,
    pub file_path: std::path::PathBuf,
    pub resource_type: crate::types::dbt::ResourceType,
    pub sql_config: SqlConfig,
    pub sql_content: String,
}

#[derive(Debug, Clone, Default)]
pub struct SqlConfig {
    pub description: Option<String>,
    pub materialized: Option<String>,
    pub tags: Vec<String>,
    pub meta: HashMap<String, String>,
    pub config_raw: String, // Store the raw config block for debugging
}

pub fn parse_sql_file(file_path: &Path, content: &str) -> Result<SqlResourceInfo> {
    let name = extract_resource_name_from_path(file_path);
    let resource_type = determine_resource_type_from_path(file_path);
    let sql_config = extract_sql_config(content)?;

    Ok(SqlResourceInfo {
        name,
        file_path: file_path.to_path_buf(),
        resource_type,
        sql_config,
        sql_content: content.to_string(),
    })
}

fn extract_resource_name_from_path(file_path: &Path) -> String {
    file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn determine_resource_type_from_path(file_path: &Path) -> crate::types::dbt::ResourceType {
    // Determine type based on directory structure
    let path_str = file_path.to_string_lossy();

    if path_str.contains("/models/") {
        crate::types::dbt::ResourceType::Model
    } else if path_str.contains("/snapshots/") {
        crate::types::dbt::ResourceType::Snapshot
    } else if path_str.contains("/macros/") {
        crate::types::dbt::ResourceType::Macro
    } else {
        // Default to model if we can't determine
        crate::types::dbt::ResourceType::Model
    }
}

fn extract_sql_config(content: &str) -> Result<SqlConfig> {
    let mut config = SqlConfig::default();

    // Look for {{ config(...) }} blocks
    if let Some(config_block) = extract_config_block(content) {
        config.config_raw = config_block.clone();

        // Parse individual config items
        config.description = extract_config_string(&config_block, "description");
        config.materialized = extract_config_string(&config_block, "materialized");
        config.tags = extract_config_array(&config_block, "tags");
        // TODO: Parse meta and other complex config items
    }

    Ok(config)
}

fn extract_config_block(content: &str) -> Option<String> {
    // Find {{ config(...) }} block - this is a simplified parser
    let mut bracket_count = 0;
    let mut in_config = false;
    let mut config_start = 0;

    let chars: Vec<char> = content.chars().collect();
    for (i, window) in chars.windows(10).enumerate() {
        let window_str: String = window.iter().collect();

        if window_str.starts_with("{{ config(") {
            in_config = true;
            config_start = i;
            bracket_count = 2; // Start with {{
        }

        if in_config {
            if chars[i] == '{' {
                bracket_count += 1;
            } else if chars[i] == '}' {
                bracket_count -= 1;
                if bracket_count == 0 {
                    // Found the end
                    return Some(chars[config_start..=i].iter().collect());
                }
            }
        }
    }

    None
}

fn extract_config_string(config_block: &str, key: &str) -> Option<String> {
    // Look for key='value' or key="value"
    let pattern = format!("{}=", key);
    if let Some(start) = config_block.find(&pattern) {
        let remaining = &config_block[start + pattern.len()..];

        // Skip whitespace
        let remaining = remaining.trim_start();

        if remaining.starts_with('"') {
            // Double quoted string
            if let Some(end) = remaining[1..].find('"') {
                return Some(remaining[1..=end].to_string());
            }
        } else if remaining.starts_with('\'') {
            // Single quoted string
            if let Some(end) = remaining[1..].find('\'') {
                return Some(remaining[1..=end].to_string());
            }
        }
    }

    None
}

fn extract_config_array(config_block: &str, key: &str) -> Vec<String> {
    // Look for key=['item1', 'item2'] or key=["item1", "item2"]
    let pattern = format!("{}=", key);
    if let Some(start) = config_block.find(&pattern) {
        let remaining = &config_block[start + pattern.len()..];
        let remaining = remaining.trim_start();

        if remaining.starts_with('[') {
            if let Some(end) = remaining.find(']') {
                let array_content = &remaining[1..end];
                return parse_simple_array(array_content);
            }
        }
    }

    Vec::new()
}

fn parse_simple_array(content: &str) -> Vec<String> {
    // Simple array parser for ['item1', 'item2']
    content
        .split(',')
        .map(|item| {
            item.trim()
                .trim_start_matches('\'')
                .trim_end_matches('\'')
                .trim_start_matches('"')
                .trim_end_matches('"')
                .to_string()
        })
        .filter(|item| !item.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_config_block() {
        let sql = r#"
        {{ config(
            materialized='table',
            description="A test model"
        ) }}
        
        SELECT * FROM raw_data
        "#;

        let config_block = extract_config_block(sql);
        assert!(config_block.is_some());
        assert!(config_block.unwrap().contains("materialized='table'"));
    }

    #[test]
    fn test_extract_config_string() {
        let config = r#"materialized='table', description="A test model""#;

        assert_eq!(
            extract_config_string(config, "materialized"),
            Some("table".to_string())
        );
        assert_eq!(
            extract_config_string(config, "description"),
            Some("A test model".to_string())
        );
    }

    #[test]
    fn test_extract_config_array() {
        let config = r#"tags=['daily', 'core'], materialized='table'"#;

        let tags = extract_config_array(config, "tags");
        assert_eq!(tags, vec!["daily", "core"]);
    }
}
