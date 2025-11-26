// src/app.rs
pub mod cli;
pub mod context;
pub mod generator;

use self::cli::{Args, Commands}; // ‼️ Imported Commands enum
use self::context::scan_directory;
use anyhow::Result;
use clap::Parser;
// ‼️ Imported specific generator functions
use self::generator::{
    generate_architecture_prompt, generate_readme_prompt, generate_refactor_prompt,
    generate_review_prompt,
};

/// Main application logic
pub fn run() -> Result<()> {
    let args = Args::parse();

    // ‼️ Scan logic remains global
    let reference_code = if let Some(path) = args.scan {
        eprintln!("🔍 Scanning directory: {:?}", path);
        scan_directory(path)?
    } else {
        String::new()
    };

    // ‼️ Match on the subcommand to dispatch to the correct generator with specific args
    let output = match &args.command {
        Commands::Architecture(cmd_args) => generate_architecture_prompt(cmd_args, &reference_code),
        Commands::CodeReview(cmd_args) => generate_review_prompt(cmd_args, &reference_code),
        Commands::Refactor(cmd_args) => generate_refactor_prompt(cmd_args, &reference_code),
        Commands::Readme(cmd_args) => generate_readme_prompt(cmd_args, &reference_code),
    };

    println!("{}", output);
    Ok(())
}
