// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::is_kdialog_available;
use crate::{BlockingDialogError, BlockingPickFilesDialog};
use std::fmt::Write;
use std::path::PathBuf;
use std::process::Command;

fn parse_zenity_multi_select(raw: &str) -> Vec<PathBuf> {
    raw.split('|').map(PathBuf::from).collect()
}

fn parse_kdialog_multi_select(raw: &str) -> Vec<PathBuf> {
    raw.split('\n').map(PathBuf::from).collect()
}

fn get_kdialog_filter(filter: &[BlockingPickFilesDialogFilter]) -> String {
    filter
        .iter()
        .map(|entry| {
            format!(
                "{} ({})",
                entry.name,
                entry
                    .extensions
                    .iter()
                    .map(|ext| format!("*.{ext}"))
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

impl<'a> BlockingPickFilesDialog<'a> {
    pub fn show(&self) -> Result<Vec<PathBuf>, BlockingDialogError> {
        if is_kdialog_available() {
            let mut args = vec!["--getopenfilename", "--title", self.title];

            if self.multiple {
                args.push("--multiple");
                args.push("--separate-output");
            }

            if !self.filter.is_empty() {
                args.push(":label1");
                args.push(get_kdialog_filter(&self.filter));
            }

            let output = Command::new("kdialog").args(args).output()?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            let trimmed = stdout.trim();

            if trimmed.is_empty() {
                Ok(Vec::new())
            } else if self.multiple {
                Ok(parse_kdialog_multi_select(trimmed))
            } else {
                Ok(vec![PathBuf::from(trimmed)])
            }
        } else {
            let mut args = vec!["--file-selection", "--title", self.title];

            if self.multiple {
                args.push("--multiple");
            }

            let output = Command::new("zenity").args(args).output()?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            let trimmed = stdout.trim();

            if trimmed.is_empty() {
                Ok(Vec::new())
            } else if self.multiple {
                Ok(parse_zenity_multi_select(trimmed))
            } else {
                Ok(vec![PathBuf::from(trimmed)])
            }
        }
    }
}
