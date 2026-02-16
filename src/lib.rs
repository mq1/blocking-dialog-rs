// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg_attr(target_os = "macos", path = "macos.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
mod os_dialog;

use raw_window_handle::WindowHandle;
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum BlockingDialogError {
    #[error("The dialog is not running on the main thread")]
    NotOnMainThread,
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
