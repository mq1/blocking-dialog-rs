// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod alert;
pub mod confirm;
pub mod pick_files;

use std::process::Command;

pub fn is_kdialog_available() -> bool {
    Command::new("which")
        .arg("kdialog")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
