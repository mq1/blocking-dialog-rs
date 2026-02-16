// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickFileDialog};
use std::path::PathBuf;
use std::process::Command;

impl<'a> BlockingPickFileDialog<'a> {
    pub fn show(&self) -> Result<PathBuf, BlockingDialogError> {
        let cmd = Command::new("zenity")
            .arg("--file-selection")
            .arg("--title")
            .arg(self.title);

        let cmd = match self.directory {
            true => cmd.arg("--directory"),
            false => cmd,
        };

        let cmd = match self.multiple {
            true => cmd.arg("--multiple"),
            false => cmd,
        };

        let output = cmd.output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let trimmed = stdout.trim();
        Ok(PathBuf::from(trimmed))
    }
}
