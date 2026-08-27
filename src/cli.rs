use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub(crate) struct Cli {
    /// Path to cards.sqlite. Defaults to the repository root.
    #[arg(long, global = true)]
    pub(crate) db: Option<PathBuf>,

    /// Path to the Comprehensive Rules Markdown file.
    #[arg(long, global = true)]
    pub(crate) rules: Option<PathBuf>,

    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    /// Report corpus sizes and input-file metadata.
    Info,
    /// Search cards by name, Oracle text, or type line.
    Cards(CardSearchArgs),
    /// Retrieve one card and, optionally, all of its rulings.
    Card(CardArgs),
    /// Search or retrieve numbered rules and glossary paragraphs.
    Rules {
        #[command(subcommand)]
        command: RulesCommand,
    },
    /// Split card text into agent-friendly structural units.
    Segment(SegmentArgs),
    /// Measure normalized ability-template frequency and coverage.
    Templates(TemplateArgs),
    /// List first-printing sets in release order with card counts.
    Sets(SetsArgs),
    /// Export and measure set-level structural audit data.
    Audit {
        #[command(subcommand)]
        command: AuditCommand,
    },
}

#[derive(Args)]
pub(crate) struct CardSearchArgs {
    /// Literal case-insensitive search query.
    pub(crate) query: String,

    /// Fields to search: name, text, type, or all.
    #[arg(long, default_value = "all", value_parser = ["name", "text", "type", "all"])]
    pub(crate) field: String,

    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u32).range(1..=500))]
    pub(crate) limit: u32,

    #[arg(long, default_value_t = 0)]
    pub(crate) offset: u32,

    /// Restrict to cards whose first printing is this set code (e.g. lea).
    #[arg(long)]
    pub(crate) set: Option<String>,

    /// Exclude the frozen protocol 6.3 held-out pool before returning rows.
    #[arg(long)]
    pub(crate) exclude_heldout: bool,
}

#[derive(Args)]
pub(crate) struct CardArgs {
    /// Exact card name (case-insensitive) or Oracle ID.
    pub(crate) query: String,

    /// Include official rulings in chronological order.
    #[arg(long)]
    pub(crate) rulings: bool,
}

#[derive(Subcommand)]
pub(crate) enum RulesCommand {
    /// Search numbered rules and glossary paragraphs.
    Search(RuleSearchArgs),
    /// Retrieve a numbered rule and all of its subrules.
    Show(RuleShowArgs),
}

#[derive(Args)]
pub(crate) struct RuleSearchArgs {
    /// Literal case-insensitive search query.
    pub(crate) query: String,

    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u32).range(1..=500))]
    pub(crate) limit: u32,
}

#[derive(Args)]
pub(crate) struct RuleShowArgs {
    /// Rule number, such as 603.1 or 704.5.
    pub(crate) id: String,
}

#[derive(Args)]
pub(crate) struct SegmentArgs {
    /// Exact card name to read from the database.
    #[arg(long, conflicts_with = "text", required_unless_present = "text")]
    pub(crate) card: Option<String>,

    /// Raw Oracle text to segment directly.
    #[arg(long, conflicts_with = "card", required_unless_present = "card")]
    pub(crate) text: Option<String>,

    /// Card name used to normalize self-references with --text.
    #[arg(long, requires = "text")]
    pub(crate) name: Option<String>,

    /// Card or face type line used for type-aware classification with --text.
    #[arg(long, requires = "text")]
    pub(crate) type_line: Option<String>,

    /// Exclude the frozen protocol 6.3 held-out pool from --card lookup.
    #[arg(long, requires = "card")]
    pub(crate) exclude_heldout: bool,
}

#[derive(Args)]
pub(crate) struct TemplateArgs {
    #[arg(long, default_value_t = 50, value_parser = clap::value_parser!(u32).range(1..=5000))]
    pub(crate) limit: u32,

    /// Omit templates occurring fewer than this many times from the result list.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..))]
    pub(crate) min_count: u32,

    /// Restrict to cards whose first printing is this set code (e.g. lea).
    #[arg(long)]
    pub(crate) set: Option<String>,
}

#[derive(Args)]
pub(crate) struct SetsArgs {
    /// Restrict to this Scryfall set_type (core, expansion, commander, ...).
    #[arg(long = "type")]
    pub(crate) set_type: Option<String>,

    /// Only include sets released on or before this date (YYYY-MM-DD).
    #[arg(long)]
    pub(crate) until: Option<String>,
}

#[derive(Subcommand)]
pub(crate) enum AuditCommand {
    /// Export every structural unit for cards first printed in a set.
    Export(SetAuditArgs),
    /// Summarize structural-unit counts for cards first printed in a set.
    Summary(SetAuditArgs),
    /// Compare a set's printed templates to chronologically earlier sets.
    Novelty(NoveltyAuditArgs),
    /// List observable suspicious candidates for manual audit triage.
    Signals(SetAuditArgs),
}

#[derive(Args, Clone)]
pub(crate) struct SetAuditArgs {
    /// First-printing set code to audit, such as lea.
    pub(crate) set: String,

    /// Exclude the frozen protocol 6.3 held-out pool before segmentation.
    #[arg(long)]
    pub(crate) exclude_heldout: bool,
}

#[derive(Args, Clone)]
pub(crate) struct NoveltyAuditArgs {
    /// First-printing set code to audit, such as arn.
    pub(crate) set: String,

    /// Earlier audited set codes to use as the comparison corpus.
    #[arg(long, value_delimiter = ',')]
    pub(crate) earlier: Vec<String>,
}
