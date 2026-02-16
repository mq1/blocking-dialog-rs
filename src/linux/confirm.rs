// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingConfirmDialog, BlockingDialogError};
use std::process::Command;

impl<'a> BlockingConfirmDialog<'a> {
    pub fn show(&self) -> Result<bool, BlockingDialogError> {
        let status = Command::new("zenity")
            .arg("--question")
            .arg("--title")
            .arg(self.title)
            .arg("--text")
            .arg(self.message)
            .status()?;

        status.success()
    }
}
