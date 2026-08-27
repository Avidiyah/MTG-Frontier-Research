mod audit;
mod cards;
mod cli;
mod database;
mod rules;
mod segment;
mod util;

#[cfg(test)]
mod tests;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::audit::command_audit;
use crate::cards::{command_card, command_cards, command_info, command_sets};
use crate::cli::{Cli, Command};
use crate::rules::command_rules;
use crate::segment::{command_segment, command_templates};

const DEFAULT_DB: &str = "cards.sqlite";
const DEFAULT_RULES: &str = "Magic-Comprehensive_Rules.md";

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let db_path = cli.db.unwrap_or_else(|| root.join(DEFAULT_DB));
    let rules_path = cli.rules.unwrap_or_else(|| root.join(DEFAULT_RULES));

    let output = match cli.command {
        Command::Info => command_info(&db_path, &rules_path)?,
        Command::Cards(args) => command_cards(&db_path, args)?,
        Command::Card(args) => command_card(&db_path, args)?,
        Command::Rules { command } => command_rules(&rules_path, command)?,
        Command::Segment(args) => command_segment(&db_path, args)?,
        Command::Templates(args) => command_templates(&db_path, args)?,
        Command::Sets(args) => command_sets(&db_path, args)?,
        Command::Audit { command } => command_audit(&db_path, command)?,
    };

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
