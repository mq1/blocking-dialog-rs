// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickFileDialog};
use std::path::PathBuf;
use std::process::Command;

impl<'a> BlockingPickFileDialog<'a> {
    pub fn show(&self) -> Result<PathBuf, BlockingDialogError> {
        let mut args = vec!["--file-selection", "--title", self.title];

        if self.directory {
            args.push("--directory");
        }

        if self.multiple {
            args.push("--multiple");
        }

        let output = Command::new("zenity").args(args).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let trimmed = stdout.trim();

        Ok(PathBuf::from(trimmed))
    }
}
