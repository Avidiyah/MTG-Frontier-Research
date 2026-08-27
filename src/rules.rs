use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use regex::Regex;
use serde::Serialize;
use serde_json::{Value, json};

use crate::cli::RulesCommand;

#[derive(Clone, Serialize)]
pub(crate) struct RuleEntry {
    pub(crate) id: Option<String>,
    pub(crate) heading: Option<String>,
    pub(crate) text: String,
    pub(crate) line: usize,
    pub(crate) kind: &'static str,
}

pub(crate) fn command_rules(path: &Path, command: RulesCommand) -> Result<Value> {
    let text = read_rules(path)?;
    let entries = parse_rules(&text);
    match command {
        RulesCommand::Search(args) => {
            let query = args.query.to_lowercase();
            let matches: Vec<_> = entries
                .into_iter()
                .filter(|entry| {
                    entry.text.to_lowercase().contains(&query)
                        || entry
                            .heading
                            .as_ref()
                            .is_some_and(|heading| heading.to_lowercase().contains(&query))
                        || entry.id.as_ref().is_some_and(|id| id == &query)
                })
                .take(args.limit as usize)
                .collect();
            Ok(json!({
                "query": args.query,
                "limit": args.limit,
                "count": matches.len(),
                "matches": matches
            }))
        }
        RulesCommand::Show(args) => {
            let requested = args.id.trim_end_matches('.');
            let matches: Vec<_> = entries
                .into_iter()
                .filter(|entry| {
                    entry
                        .id
                        .as_ref()
                        .is_some_and(|id| is_rule_within(id, requested))
                })
                .collect();
            if matches.is_empty() {
                bail!("rule {:?} was not found", args.id);
            }
            Ok(json!({"rule": args.id, "entries": matches}))
        }
    }
}

pub(crate) fn is_rule_within(candidate: &str, requested: &str) -> bool {
    if candidate == requested {
        return true;
    }
    candidate
        .strip_prefix(requested)
        .and_then(|suffix| suffix.chars().next())
        .is_some_and(|first| first == '.' || first.is_ascii_lowercase())
}

pub(crate) fn read_rules(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("failed to read Comprehensive Rules: {}", path.display()))
}

pub(crate) fn parse_rules(text: &str) -> Vec<RuleEntry> {
    let numbered =
        Regex::new(r"^(\d{3}(?:\.\d+[a-z]?)*)\.?\s+(.+)$").expect("valid numbered-rule regex");
    let lines: Vec<_> = text.lines().collect();
    let glossary_start = lines
        .iter()
        .rposition(|line| *line == "Glossary")
        .unwrap_or(lines.len());
    let mut entries = Vec::new();

    for (index, line) in lines[..glossary_start].iter().enumerate() {
        let trimmed = line.trim();
        if let Some(captures) = numbered.captures(trimmed) {
            entries.push(RuleEntry {
                id: Some(captures[1].to_owned()),
                heading: None,
                text: captures[2].to_owned(),
                line: index + 1,
                kind: "rule",
            });
        }
    }

    let mut index = glossary_start + 1;
    while index < lines.len() {
        while index < lines.len() && lines[index].trim().is_empty() {
            index += 1;
        }
        if index >= lines.len() {
            break;
        }
        let heading_line = index;
        let heading = lines[index].trim().to_owned();
        index += 1;
        let mut paragraphs = Vec::new();
        while index < lines.len() && !lines[index].trim().is_empty() {
            paragraphs.push(lines[index].trim());
            index += 1;
        }
        entries.push(RuleEntry {
            id: None,
            heading: Some(heading),
            text: paragraphs.join(" "),
            line: heading_line + 1,
            kind: "glossary",
        });
    }
    entries
}
