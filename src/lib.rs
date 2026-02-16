// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
#[cfg_attr(target_os = "linux", path = "linux/mod.rs")]
#[cfg_attr(target_os = "windows", path = "windows/mod.rs")]
mod os_dialog;

use raw_window_handle::WindowHandle;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockingDialogError {
    #[error("The dialog is not running on the main thread")]
    NotOnMainThread,
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Clone, Copy)]
pub enum BlockingDialogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct BlockingAlertDialog<'a> {
    pub window: Option<WindowHandle<'a>>,
    pub title: &'a str,
    pub message: &'a str,
    pub level: BlockingDialogLevel,
}
