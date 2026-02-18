// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickDirectoryDialog};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use rfd::FileDialog;
use std::path::PathBuf;

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingPickDirectoryDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let result = FileDialog::new()
            .set_title(self.title)
            .set_parent(&self.window)
            .pick_folder();

        Ok(result)
    }
}
