// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
#[cfg_attr(target_os = "linux", path = "linux/mod.rs")]
#[cfg_attr(target_os = "windows", path = "windows/mod.rs")]
mod os_dialog;

use raw_window_handle::{HandleError, HasWindowHandle};
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockingDialogError {
    #[error("The dialog is not running on the main thread")]
    NotOnMainThread,
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Handle error: {0}")]
    Handle(HandleError),
}

#[derive(Debug, Clone, Copy)]
pub enum BlockingDialogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct BlockingAlertDialog<'a, W: HasWindowHandle> {
    pub window: W,
    pub title: &'a str,
    pub message: &'a str,
    pub level: BlockingDialogLevel,
}

#[derive(Debug, Clone)]
pub struct BlockingConfirmDialog<'a, W: HasWindowHandle> {
    pub window: W,
    pub title: &'a str,
    pub message: &'a str,
    pub level: BlockingDialogLevel,
}

#[derive(Debug, Clone)]
pub struct BlockingPickFilesDialogFilter<'a> {
    pub name: &'a str,
    pub extensions: &'a [&'a str],
}

#[derive(Debug, Clone)]
pub struct BlockingPickFilesDialog<'a, W: HasWindowHandle> {
    pub window: W,
    pub title: &'a str,
    pub multiple: bool,
    pub filter: &'a [BlockingPickFilesDialogFilter<'a>],
}

#[derive(Debug, Clone)]
pub struct BlockingPickDirectoryDialog<'a, W: HasWindowHandle> {
    pub window: W,
    pub title: &'a str,
}

#[derive(Debug, Clone)]
pub struct BlockingSaveFileDialog<'a, W: HasWindowHandle> {
    pub window: W,
    pub title: &'a str,
    pub default_filename: Option<&'a str>,
    pub filter: &'a [BlockingPickFilesDialogFilter<'a>],
}
