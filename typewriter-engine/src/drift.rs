//! Drift detection between expected generated output and on-disk files.

use anyhow::Result;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

use crate::emit::{GeneratedFile, file_extension, language_label, output_dir_for_language};
use crate::{Language, TypewriterConfig, all_languages};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftStatus {
    UpToDate,
    OutOfSync,
    Missing,
    Orphaned,
}

#[derive(Debug, Clone, Serialize)]
pub struct DriftEntry {
    pub type_name: String,
    pub language: String,
    pub output_path: String,
    pub status: DriftStatus,
    pub reason: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DriftSummary {
    pub up_to_date: usize,
    pub out_of_sync: usize,
    pub missing: usize,
    pub orphaned: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DriftReport {
    pub project_root: String,
    pub generated_at: String,
    pub summary: DriftSummary,
    pub entries: Vec<DriftEntry>,
}

pub fn build_drift_report(
    expected_files: &[GeneratedFile],
    project_root: &Path,
    config: &TypewriterConfig,
    language_scope: &[Language],
) -> Result<DriftReport> {
    let mut entries = Vec::new();
    let mut expected_by_path = BTreeMap::new();

    for file in expected_files {
        expected_by_path.insert(file.output_path.clone(), file);
    }

    for (path, file) in &expected_by_path {
        let status = if !path.exists() {
            DriftStatus::Missing
        } else {
            let existing = std::fs::read_to_string(path).unwrap_or_default();
            if existing == file.content {
                DriftStatus::UpToDate
            } else {
                DriftStatus::OutOfSync
            }
        };

        let reason = match status {
            DriftStatus::UpToDate => "generated content matches existing file",
            DriftStatus::OutOfSync => "existing file differs from generated content",
            DriftStatus::Missing => "expected generated file does not exist",
            DriftStatus::Orphaned => "",
        }
        .to_string();

        entries.push(DriftEntry {
            type_name: file.type_name.clone(),
            language: language_label(file.language).to_string(),
            output_path: rel_path(project_root, path),
            status,
            reason,
        });
    }

    let scope = if language_scope.is_empty() {
        all_languages()
    } else {
        language_scope.to_vec()
    };

    let expected_paths: BTreeSet<_> = expected_by_path.keys().cloned().collect();

    for language in scope {
        let output_dir = project_root.join(output_dir_for_language(config, language));
        let ext = file_extension(language);

        if !output_dir.exists() {
            continue;
        }

        for entry in WalkDir::new(&output_dir).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }

            if entry
                .path()
                .extension()
                .map(|found| found == ext)
                .unwrap_or(false)
                && !expected_paths.contains(entry.path())
                && is_typewriter_generated_file(entry.path())
            {
                let type_name = entry
                    .path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                entries.push(DriftEntry {
                    type_name,
                    language: language_label(language).to_string(),
                    output_path: rel_path(project_root, entry.path()),
                    status: DriftStatus::Orphaned,
                    reason:
                        "generated file exists but no matching Rust TypeWriter source was found"
                            .to_string(),
                });
            }
        }
    }

    entries.sort_by(|a, b| a.output_path.cmp(&b.output_path));

    let summary = summarize(&entries);

    let generated_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());

    Ok(DriftReport {
        project_root: project_root.display().to_string(),
        generated_at,
        summary,
        entries,
    })
}

pub fn summarize(entries: &[DriftEntry]) -> DriftSummary {
    let mut summary = DriftSummary::default();
    for entry in entries {
        match entry.status {
            DriftStatus::UpToDate => summary.up_to_date += 1,
            DriftStatus::OutOfSync => summary.out_of_sync += 1,
            DriftStatus::Missing => summary.missing += 1,
            DriftStatus::Orphaned => summary.orphaned += 1,
        }
    }
    summary
}

pub fn has_drift(summary: &DriftSummary) -> bool {
    summary.out_of_sync > 0 || summary.missing > 0 || summary.orphaned > 0
}

fn rel_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn is_typewriter_generated_file(path: &Path) -> bool {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return false,
    };

    content.contains("generated by typewriter")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_status_counts() {
        let entries = vec![
            DriftEntry {
                type_name: "A".into(),
                language: "typescript".into(),
                output_path: "a.ts".into(),
                status: DriftStatus::UpToDate,
                reason: String::new(),
            },
            DriftEntry {
                type_name: "B".into(),
                language: "typescript".into(),
                output_path: "b.ts".into(),
                status: DriftStatus::OutOfSync,
                reason: String::new(),
            },
            DriftEntry {
                type_name: "C".into(),
                language: "python".into(),
                output_path: "c.py".into(),
                status: DriftStatus::Missing,
                reason: String::new(),
            },
            DriftEntry {
                type_name: "D".into(),
                language: "go".into(),
                output_path: "d.go".into(),
                status: DriftStatus::Orphaned,
                reason: String::new(),
            },
        ];

        let summary = summarize(&entries);
        assert_eq!(summary.up_to_date, 1);
        assert_eq!(summary.out_of_sync, 1);
        assert_eq!(summary.missing, 1);
        assert_eq!(summary.orphaned, 1);
        assert!(has_drift(&summary));
    }
}
