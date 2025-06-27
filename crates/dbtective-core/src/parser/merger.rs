use crate::parser::{sql::SqlResourceInfo, yaml::YamlResourceConfig};
use crate::types::dbt::{DbtResource, Model, ResourceType, Source};
use anyhow::Result;
use std::collections::HashMap;

pub fn merge_configurations(
    sql_resources: HashMap<String, SqlResourceInfo>,
    yaml_configs: HashMap<String, YamlResourceConfig>,
) -> Result<Vec<Box<dyn DbtResource>>> {
    let mut resources = Vec::new();

    // Get all unique resource names from both sources
    let mut all_names = std::collections::HashSet::new();
    all_names.extend(sql_resources.keys().cloned());
    all_names.extend(yaml_configs.keys().cloned());

    for name in all_names {
        let sql_info = sql_resources.get(&name);
        let yaml_config = yaml_configs.get(&name);

        match (sql_info, yaml_config) {
            // Resource has both SQL and YAML config
            (Some(sql), Some(yaml)) => {
                let merged = merge_sql_and_yaml_config(sql, yaml)?;
                resources.push(merged);
            }
            // Resource only has SQL config
            (Some(sql), None) => {
                let resource = create_resource_from_sql_only(sql)?;
                resources.push(resource);
            }
            // Resource only has YAML config (e.g., sources)
            (None, Some(yaml)) => {
                let resource = create_resource_from_yaml_only(yaml)?;
                resources.push(resource);
            }
            // This shouldn't happen given our logic above
            (None, None) => unreachable!(),
        }
    }

    Ok(resources)
}

fn merge_sql_and_yaml_config(
    sql: &SqlResourceInfo,
    yaml: &YamlResourceConfig,
) -> Result<Box<dyn DbtResource>> {
    match sql.resource_type {
        ResourceType::Model => {
            let mut model = Model::new(sql.name.clone());

            // YAML description takes precedence over SQL description
            model.description = yaml
                .description
                .clone()
                .or_else(|| sql.sql_config.description.clone());

            // Merge tags (YAML + SQL)
            model.tags = merge_tags(&yaml.tags, &sql.sql_config.tags);

            // YAML tests take precedence
            model.tests = yaml.tests.clone();

            // Merge meta (YAML takes precedence for conflicts)
            model.meta = merge_meta(&yaml.meta, &sql.sql_config.meta);

            // SQL-specific config
            model.materialized = sql.sql_config.materialized.clone();
            model.sql_file = Some(sql.file_path.clone());
            model.yaml_file = Some(yaml.file_path.clone());

            Ok(Box::new(model))
        }
        ResourceType::Source => {
            // Sources typically only have YAML config, but handle edge cases
            create_source_table_from_configs(sql, yaml)
        }
        _ => {
            // TODO: Handle other resource types (snapshots, seeds, etc.)
            create_resource_from_sql_only(sql)
        }
    }
}

fn create_resource_from_sql_only(sql: &SqlResourceInfo) -> Result<Box<dyn DbtResource>> {
    match sql.resource_type {
        ResourceType::Model => {
            let mut model = Model::new(sql.name.clone());
            model.description = sql.sql_config.description.clone();
            model.tags = sql.sql_config.tags.clone();
            model.meta = sql.sql_config.meta.clone();
            model.materialized = sql.sql_config.materialized.clone();
            model.sql_file = Some(sql.file_path.clone());

            Ok(Box::new(model))
        }
        _ => {
            // For now, treat other types as models
            create_resource_from_sql_only(&SqlResourceInfo {
                resource_type: ResourceType::Model,
                ..sql.clone()
            })
        }
    }
}

fn create_resource_from_yaml_only(yaml: &YamlResourceConfig) -> Result<Box<dyn DbtResource>> {
    match yaml.resource_type {
        ResourceType::Source => {
            let mut source_table = Source::new(
                yaml.name.clone(),
                yaml.source_name.clone().unwrap_or_default(),
            );
            source_table.description = yaml.description.clone();
            source_table.tags = yaml.tags.clone();
            source_table.tests = yaml.tests.clone();
            source_table.meta = yaml.meta.clone();
            source_table.yaml_file = Some(yaml.file_path.to_string_lossy().to_string());

            Ok(Box::new(source_table))
        }
        ResourceType::Model => {
            let mut model = Model::new(yaml.name.clone());
            model.description = yaml.description.clone();
            model.tags = yaml.tags.clone();
            model.tests = yaml.tests.clone();
            model.meta = yaml.meta.clone();
            model.yaml_file = Some(yaml.file_path.clone());

            Ok(Box::new(model))
        }
        _ => {
            // TODO: Handle other resource types
            create_resource_from_yaml_only(&YamlResourceConfig {
                resource_type: ResourceType::Model,
                ..yaml.clone()
            })
        }
    }
}

fn create_source_table_from_configs(
    sql: &SqlResourceInfo,
    yaml: &YamlResourceConfig,
) -> Result<Box<dyn DbtResource>> {
    let mut source_table = Source::new(
        yaml.name.clone(),
        yaml.source_name.clone().unwrap_or_default(),
    );

    // YAML takes precedence for sources
    source_table.description = yaml.description.clone();
    source_table.tags = yaml.tags.clone();
    source_table.tests = yaml.tests.clone();
    source_table.meta = yaml.meta.clone();
    source_table.yaml_file = Some(yaml.file_path.to_string_lossy().to_string());
    Ok(Box::new(source_table))
}

fn merge_tags(yaml_tags: &[String], sql_tags: &[String]) -> Vec<String> {
    let mut tags = yaml_tags.to_vec();

    // Add SQL tags that aren't already in YAML tags
    for sql_tag in sql_tags {
        if !tags.contains(sql_tag) {
            tags.push(sql_tag.clone());
        }
    }

    tags
}

fn merge_meta(
    yaml_meta: &HashMap<String, String>,
    sql_meta: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut meta = sql_meta.clone();

    // YAML meta takes precedence for conflicts
    meta.extend(yaml_meta.clone());

    meta
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_merge_tags() {
        let yaml_tags = vec!["yaml_tag".to_string(), "shared_tag".to_string()];
        let sql_tags = vec!["sql_tag".to_string(), "shared_tag".to_string()];

        let merged = merge_tags(&yaml_tags, &sql_tags);

        assert_eq!(merged.len(), 3);
        assert!(merged.contains(&"yaml_tag".to_string()));
        assert!(merged.contains(&"sql_tag".to_string()));
        assert!(merged.contains(&"shared_tag".to_string()));
    }

    #[test]
    fn test_merge_meta() {
        let mut yaml_meta = HashMap::new();
        yaml_meta.insert("yaml_key".to_string(), "yaml_value".to_string());
        yaml_meta.insert("shared_key".to_string(), "yaml_wins".to_string());

        let mut sql_meta = HashMap::new();
        sql_meta.insert("sql_key".to_string(), "sql_value".to_string());
        sql_meta.insert("shared_key".to_string(), "sql_loses".to_string());

        let merged = merge_meta(&yaml_meta, &sql_meta);

        assert_eq!(merged.len(), 3);
        assert_eq!(merged.get("yaml_key"), Some(&"yaml_value".to_string()));
        assert_eq!(merged.get("sql_key"), Some(&"sql_value".to_string()));
        assert_eq!(merged.get("shared_key"), Some(&"yaml_wins".to_string())); // YAML wins
    }
}
