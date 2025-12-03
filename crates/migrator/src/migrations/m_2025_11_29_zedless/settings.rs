use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::SETTINGS_LANGUAGES_PATTERN;

pub const SETTINGS_PATTERNS: MigrationPatterns =
    &[(SETTINGS_LANGUAGES_PATTERN, migrate_prettier_to_auto)];

fn migrate_prettier_to_auto(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let name_ix = query.capture_index_for_name("setting_name")?;
    let name_range = mat.nodes_for_capture_index(name_ix).next()?.byte_range();
    let name = contents.get(name_range)?;

    if name != "formatter" {
        return None;
    }

    let value_ix = query.capture_index_for_name("value")?;
    let value_node = mat.nodes_for_capture_index(value_ix).next()?;
    let value_range = value_node.byte_range();
    let value = contents.get(value_range.clone())?;

    match value {
        "\"prettier\"" => {
            let replacement = "\"auto\"".to_string();
            Some((value_range, replacement))
        }
        _ => None,
    }
}
