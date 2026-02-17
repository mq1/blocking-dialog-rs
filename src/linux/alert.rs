// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::is_kdialog_available;
use crate::{BlockingAlertDialog, BlockingDialogError, BlockingDialogLevel};
use raw_window_handle::HasWindowHandle;
use std::process::Command;

fn get_kdialog_level_flag(level: BlockingDialogLevel) -> &'static str {
    match level {
        BlockingDialogLevel::Info => "--msgbox",
        BlockingDialogLevel::Warning => "--sorry",
        BlockingDialogLevel::Error => "--error",
    }
}

fn get_zenity_level_flag(level: BlockingDialogLevel) -> &'static str {
    match level {
        BlockingDialogLevel::Info => "--info",
        BlockingDialogLevel::Warning => "--warning",
        BlockingDialogLevel::Error => "--error",
    }
}

impl<'a, W: HasWindowHandle> BlockingAlertDialog<'a, W> {
    pub fn show(&self) -> Result<(), BlockingDialogError> {
        let _ = if is_kdialog_available() {
            Command::new("kdialog")
                .arg("--title")
                .arg(self.title)
                .arg(get_kdialog_level_flag(self.level))
                .arg(self.message)
                .status()?
        } else {
            Command::new("zenity")
                .arg(get_zenity_level_flag(self.level))
                .arg("--title")
                .arg(self.title)
                .arg("--text")
                .arg(self.message)
                .status()?
        };

        Ok(())
    }
}
