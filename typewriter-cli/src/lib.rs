//! # typebridge-cli
//!
//! Standalone CLI for typewriter type synchronization.
//!
//! This crate provides the `typewriter` and `cargo-typewriter` command-line tools
//! for generating, checking, and watching cross-language type definitions.
//!
//! ## Installation
//!
//! ### Pre-built Binary
//!
//! ```bash
//! cargo install typebridge-cli
//! ```
//!
//! ### As Cargo Plugin
//!
//! After installing, run via cargo:
//!
//! ```bash
//! cargo install typebridge-cli
//! cargo typewriter --help
//! ```
//!
//! ## Commands
//!
//! ### `typewriter generate`
//!
//! Generate type files from Rust source definitions.
//!
//! ```bash
//! # Generate from a single file
//! typewriter generate src/models.rs
//!
//! # Generate from all Rust files in the project
//! typewriter generate --all
//!
//! # Generate only TypeScript and Python
//! typewriter generate --all --lang typescript,python
//!
//! # Show unified diffs for changed files
//! typewriter generate --all --diff
//! ```
//!
//! ### `typewriter check`
//!
//! Check if generated files are in sync with Rust source.
//!
//! ```bash
//! # Check all types
//! typewriter check
//!
//! # Check with CI exit code (non-zero on drift)
//! typewriter check --ci
//!
//! # Output as JSON
//! typewriter check --json
//!
//! # Write JSON report to file
//! typewriter check --json-out drift-report.json
//!
//! # Check specific languages
//! typewriter check --lang typescript,python
//! ```
//!
//! ### `typewriter watch`
//!
//! Watch Rust files and regenerate types on save.
//!
//! ```bash
//! # Watch src directory (default)
//! typewriter watch
//!
//! # Watch custom directory
//! typewriter watch src/models/
//!
//! # Watch with specific languages
//! typewriter watch --lang typescript,python
//!
//! # Set debounce interval (milliseconds)
//! typewriter watch --debounce-ms 100
//! ```
//!
//! ## Exit Codes
//!
//! | Code | Meaning |
//! |------|---------|
//! | 0 | Success (no drift for `check --ci`) |
//! | 1 | Error or drift detected (for `check --ci`) |
//!
//! ## Configuration
//!
//! The CLI respects `typewriter.toml` in the project root for output directories,
//! file naming styles, and other configuration options. See the main typewriter
//! documentation for configuration details.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use similar::TextDiff;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use typewriter_engine::drift::{self, DriftStatus};
use typewriter_engine::emit::{self, language_label};
use typewriter_engine::{parse_languages, project, scan};

#[derive(Parser, Debug)]
#[command(
    name = "typewriter",
    about = "Generate and verify cross-language types"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate type files from Rust source definitions.
    Generate {
        /// Generate from a single Rust source file.
        file: Option<PathBuf>,
        /// Generate from all Rust files in project root.
        #[arg(long)]
        all: bool,
        /// Restrict generation to languages (comma-separated).
        #[arg(long, value_delimiter = ',')]
        lang: Vec<String>,
        /// Show unified diffs for changed files.
        #[arg(long)]
        diff: bool,
    },
    /// Check if generated files are in sync with Rust source.
    Check {
        /// Exit with code 1 if drift is detected.
        #[arg(long)]
        ci: bool,
        /// Print structured JSON drift report to stdout.
        #[arg(long)]
        json: bool,
        /// Write structured JSON drift report to a file.
        #[arg(long)]
        json_out: Option<PathBuf>,
        /// Restrict check to languages (comma-separated).
        #[arg(long, value_delimiter = ',')]
        lang: Vec<String>,
    },
    /// Watch Rust files and regenerate on save.
    Watch {
        /// Directory to watch recursively (default: ./src).
        path: Option<PathBuf>,
        /// Restrict generation to languages (comma-separated).
        #[arg(long, value_delimiter = ',')]
        lang: Vec<String>,
        /// Debounce interval for filesystem events.
        #[arg(long, default_value_t = 50)]
        debounce_ms: u64,
    },
}

pub fn run() -> Result<i32> {
    run_with_args(std::env::args_os())
}

pub fn run_with_args<I, T>(args: I) -> Result<i32>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = Cli::try_parse_from(args).map_err(|err| anyhow::anyhow!(err.to_string()))?;

    match cli.command {
        Commands::Generate {
            file,
            all,
            lang,
            diff,
        } => cmd_generate(file, all, lang, diff),
        Commands::Check {
            ci,
            json,
            json_out,
            lang,
        } => cmd_check(ci, json, json_out, lang),
        Commands::Watch {
            path,
            lang,
            debounce_ms,
        } => cmd_watch(path, lang, debounce_ms),
    }
}

fn cmd_generate(file: Option<PathBuf>, all: bool, lang: Vec<String>, diff: bool) -> Result<i32> {
    if all == file.is_some() {
        anyhow::bail!("use exactly one input mode: either `generate <file>` or `generate --all`");
    }

    let cwd = std::env::current_dir()?;
    let project_root = project::discover_project_root(&cwd)?;
    let config = project::load_config(&project_root).unwrap_or_default();
    let lang_filter = parse_languages(&lang)?;

    let specs = if all {
        scan::scan_project(&project_root)?
    } else {
        let source = resolve_input_path(file.expect("validated"), &cwd);
        scan::scan_file(&source)?
    };

    let rendered = emit::render_specs_deduped(&specs, &project_root, &config, &lang_filter, false)?;

    let started = Instant::now();
    let mut updated = 0usize;
    let mut created = 0usize;
    let mut unchanged = 0usize;

    let mut before_contents = BTreeMap::new();
    for file in &rendered {
        if let Ok(existing) = std::fs::read_to_string(&file.output_path) {
            before_contents.insert(file.output_path.clone(), existing);
        }
    }

    emit::write_generated_files(&rendered)?;

    for file in &rendered {
        let rel = rel_path(&project_root, &file.output_path);
        match before_contents.get(&file.output_path) {
            None => {
                created += 1;
                eprintln!(
                    "{} {} [{}]",
                    "Created".green(),
                    rel,
                    language_label(file.language)
                );
                if diff {
                    print_diff(&project_root, &file.output_path, "", &file.content);
                }
            }
            Some(existing) if existing == &file.content => {
                unchanged += 1;
                eprintln!(
                    "{} {} [{}]",
                    "Unchanged".bright_black(),
                    rel,
                    language_label(file.language)
                );
            }
            Some(existing) => {
                updated += 1;
                eprintln!(
                    "{} {} [{}]",
                    "Updated".yellow(),
                    rel,
                    language_label(file.language)
                );
                if diff {
                    print_diff(&project_root, &file.output_path, existing, &file.content);
                }
            }
        }
    }

    eprintln!(
        "{} in {}ms (created: {}, updated: {}, unchanged: {})",
        "Generation complete".bold(),
        started.elapsed().as_millis(),
        created,
        updated,
        unchanged
    );

    Ok(0)
}

fn cmd_check(ci: bool, json: bool, json_out: Option<PathBuf>, lang: Vec<String>) -> Result<i32> {
    let cwd = std::env::current_dir()?;
    let project_root = project::discover_project_root(&cwd)?;
    let config = project::load_config(&project_root).unwrap_or_default();
    let lang_filter = parse_languages(&lang)?;

    let specs = scan::scan_project(&project_root)?;
    let rendered = emit::render_specs_deduped(&specs, &project_root, &config, &lang_filter, false)?;

    let report = drift::build_drift_report(&rendered, &project_root, &config, &lang_filter)?;

    if !json {
        print_human_report(&report);
    } else {
        let output = serde_json::to_string_pretty(&report)?;
        println!("{}", output);
    }

    if let Some(path) = json_out {
        let full = resolve_input_path(path, &cwd);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&full, serde_json::to_string_pretty(&report)?)
            .with_context(|| format!("failed to write json report to {}", full.display()))?;
        eprintln!("{} {}", "Wrote JSON report: ".green(), full.display());
    }

    if ci && drift::has_drift(&report.summary) {
        eprintln!("{} drift detected in CI mode", "Error:".red().bold());
        return Ok(1);
    }

    Ok(0)
}

fn cmd_watch(path: Option<PathBuf>, lang: Vec<String>, debounce_ms: u64) -> Result<i32> {
    let cwd = std::env::current_dir()?;
    let project_root = project::discover_project_root(&cwd)?;
    let watch_root = resolve_input_path(path.unwrap_or_else(|| PathBuf::from("src")), &cwd);
    let config = project::load_config(&project_root).unwrap_or_default();
    let lang_filter = parse_languages(&lang)?;

    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(
        move |result| {
            let _ = tx.send(result);
        },
        notify::Config::default(),
    )?;

    watcher.watch(&watch_root, RecursiveMode::Recursive)?;

    eprintln!(
        "{} {} (debounce={}ms)",
        "Watching".green().bold(),
        watch_root.display(),
        debounce_ms
    );

    loop {
        let first = match rx.recv() {
            Ok(event) => event,
            Err(err) => {
                eprintln!("{} watcher channel closed: {}", "Error:".red(), err);
                return Ok(1);
            }
        };

        let mut changed_files = BTreeSet::new();
        collect_changed_rust_files(first, &mut changed_files);

        while let Ok(event) = rx.recv_timeout(Duration::from_millis(debounce_ms)) {
            collect_changed_rust_files(event, &mut changed_files);
        }

        if changed_files.is_empty() {
            continue;
        }

        let batch_started = Instant::now();
        let mut specs = Vec::new();

        for changed in &changed_files {
            eprintln!("{} {}", "Changed:".cyan(), rel_path(&project_root, changed));
            if changed.exists() {
                match scan::scan_file(changed) {
                    Ok(mut found) => specs.append(&mut found),
                    Err(err) => eprintln!("{} {}", "Scan error:".red(), err),
                }
            }
        }

        if specs.is_empty() {
            continue;
        }

        let mut names: Vec<_> = specs
            .iter()
            .map(|s| s.type_def.name().to_string())
            .collect();
        names.sort();
        names.dedup();
        for name in names {
            eprintln!("{} {}", "Detected TypeWriter type:".blue(), name);
        }

        let rendered =
            emit::render_specs_deduped(&specs, &project_root, &config, &lang_filter, false)?;

        let mut updated = 0usize;
        for file in rendered {
            let before = std::fs::read_to_string(&file.output_path).ok();
            emit::write_generated_files(std::slice::from_ref(&file))?;

            let changed = before.map(|c| c != file.content).unwrap_or(true);
            if changed {
                updated += 1;
            }

            eprintln!(
                "{} {} [{}]",
                "Regenerated".green(),
                rel_path(&project_root, &file.output_path),
                language_label(file.language)
            );
        }

        eprintln!(
            "{} {} file(s) in {}ms",
            "Done".bold(),
            updated,
            batch_started.elapsed().as_millis()
        );
    }
}

fn print_human_report(report: &drift::DriftReport) {
    for entry in &report.entries {
        let symbol = match entry.status {
            DriftStatus::UpToDate => "OK".green(),
            DriftStatus::OutOfSync => "DRIFT".yellow(),
            DriftStatus::Missing => "MISSING".red(),
            DriftStatus::Orphaned => "ORPHAN".magenta(),
        };

        eprintln!(
            "{} {} [{}] - {}",
            symbol, entry.output_path, entry.language, entry.reason
        );
    }

    eprintln!(
        "{} up_to_date={}, out_of_sync={}, missing={}, orphaned={}",
        "Summary:".bold(),
        report.summary.up_to_date,
        report.summary.out_of_sync,
        report.summary.missing,
        report.summary.orphaned
    );
}

fn print_diff(project_root: &Path, path: &Path, before: &str, after: &str) {
    let rel = rel_path(project_root, path);
    let diff = TextDiff::from_lines(before, after)
        .unified_diff()
        .context_radius(3)
        .header(&format!("a/{}", rel), &format!("b/{}", rel))
        .to_string();

    if !diff.trim().is_empty() {
        println!("{}", diff);
    }
}

fn collect_changed_rust_files(event: Result<Event, notify::Error>, files: &mut BTreeSet<PathBuf>) {
    let event = match event {
        Ok(event) => event,
        Err(err) => {
            eprintln!("{} {}", "Watch error:".red(), err);
            return;
        }
    };

    for path in event.paths {
        if path.extension().map(|ext| ext == "rs").unwrap_or(false) {
            files.insert(path);
        }
    }
}

fn rel_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn resolve_input_path(path: PathBuf, cwd: &Path) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        cwd.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_generate_mode() {
        let err = run_with_args(["typewriter", "generate"]).unwrap_err();
        assert!(err.to_string().contains("use exactly one input mode"));
    }

    #[test]
    fn parses_comma_separated_langs() {
        let parsed = parse_languages(&["typescript,python".to_string()]).unwrap();
        assert_eq!(
            parsed,
            vec![
                typewriter_engine::Language::TypeScript,
                typewriter_engine::Language::Python
            ]
        );
    }
}
