// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::is_kdialog_available;
use crate::{BlockingDialogError, BlockingPickDirectoryDialog};
use raw_window_handle::HasWindowHandle;
use std::path::PathBuf;
use std::process::Command;

impl<'a, W: HasWindowHandle> BlockingPickDirectoryDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        if is_kdialog_available() {
            let output = Command::new("kdialog")
                .args(["--title", self.title, "--getexistingdirectory"])
                .output()?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            let trimmed = stdout.trim();

            if trimmed.is_empty() {
                Ok(None)
            } else {
                Ok(Some(PathBuf::from(trimmed)))
            }
        } else {
            let output = Command::new("zenity")
                .args(["--file-selection", "--directory", "--title", self.title])
                .output()?;

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
