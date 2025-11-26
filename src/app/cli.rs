use clap::{Parser, Subcommand}; // ‼️ Changed ValueEnum to Subcommand
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory to scan for code context (Global argument)
    #[arg(long)]
    pub scan: Option<PathBuf>,

    /// Read input from Stdin (Global argument)
    #[arg(long)]
    pub stdin: bool,

    // ‼️ Replaced the 'mode' and generic optional fields with a Subcommand
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)] // ‼️ Derived Subcommand instead of ValueEnum
pub enum Commands {
    /// Generate a full project architecture and implementation plan
    Architecture(ArchitectureArgs),

    /// Generate a prompt for reviewing existing code
    CodeReview(CodeReviewArgs),

    /// Generate a prompt for refactoring specific logic
    Refactor(RefactorArgs),

    /// Generate a prompt for a README file
    Readme(ReadmeArgs),
}

// ‼️ Defined specific structs for each command to allow custom args per generator

#[derive(Parser, Debug)]
pub struct ArchitectureArgs {
    /// The main idea or description of the project
    #[arg(short, long)]
    pub description: Option<String>,

    /// Specific constraints or library requirements
    #[arg(short, long)]
    pub context: Option<String>,
}

#[derive(Parser, Debug)]
pub struct CodeReviewArgs {
    /// What specific area to focus the review on
    #[arg(short, long)]
    pub focus: Option<String>,
}

#[derive(Parser, Debug)]
pub struct RefactorArgs {
    /// The specific goal of the refactor (e.g., "Modernize error handling")
    #[arg(short, long)]
    pub goal: String,
}

#[derive(Parser, Debug)]
pub struct ReadmeArgs {
    /// Tone of the readme (Specific to this generator only!)
    #[arg(long, default_value = "Professional and Concise")]
    pub style: String,

    /// Extra details to include
    #[arg(short, long)]
    pub details: Option<String>,
}
