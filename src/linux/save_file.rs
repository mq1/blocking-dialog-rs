// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::is_kdialog_available;
use crate::{BlockingDialogError, BlockingPickFilesDialogFilter, BlockingSaveFileDialog};
use raw_window_handle::HasWindowHandle;
use std::path::PathBuf;
use std::process::Command;

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

impl<'a, W: HasWindowHandle> BlockingSaveFileDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        if is_kdialog_available() {
            let filter = get_kdialog_filter(self.filter);

            let mut args = vec!["--getsavefilename", "--title", self.title];

            if let Some(default_filename) = self.default_filename {
                args.push(default_filename);
            }

            if !self.filter.is_empty() {
                args.push(":label1");
                args.push(&filter);
            }

            let output = Command::new("kdialog").args(args).output()?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            let trimmed = stdout.trim();

            if trimmed.is_empty() {
                Ok(None)
            } else {
                Ok(Some(PathBuf::from(trimmed)))
            }
        } else {
            let mut filter_args = Vec::new();

            for entry in &self.filter {
                let patterns = entry
                    .extensions
                    .iter()
                    .map(|ext| format!("*.{ext}"))
                    .collect::<Vec<_>>()
                    .join(" ");
                filter_args.push(format!("{} | {}", entry.name, patterns));
            }

            let mut args = vec!["--file-selection", "--save", "--title", self.title];

            if let Some(default_filename) = self.default_filename {
                args.push("--filename");
                args.push(default_filename);
            }

            for filter_arg in &filter_args {
                args.push("--file-filter");
                args.push(filter_arg.as_str());
            }

            let output = Command::new("zenity").args(args).output()?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            let trimmed = stdout.trim();

            if trimmed.is_empty() {
                Ok(None)
            } else {
                Ok(Some(PathBuf::from(trimmed)))
            }
        }
    }
}
