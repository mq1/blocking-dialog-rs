// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingAlertDialog, BlockingDialogError, BlockingDialogLevel};
use std::process::Command;

fn get_level_flag(level: BlockingDialogLevel) -> &'static str {
    match level {
        BlockingDialogLevel::Info => "--info",
        BlockingDialogLevel::Warning => "--warning",
        BlockingDialogLevel::Error => "--error",
    }
}

impl<'a> BlockingAlertDialog<'a> {
    pub fn show(&self) -> Result<(), BlockingDialogError> {
        let level_flag = get_level_flag(self.level);

        let cmd = Command::new("zenity")
            .arg(level_flag)
            .arg("--title")
            .arg(self.title)
            .arg("--text")
            .arg(self.message);

        let _ = cmd.status();

        Ok(())
    }
}
