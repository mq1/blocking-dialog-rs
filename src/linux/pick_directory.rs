// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickDirectoryDialog};
use native_dialog::DialogBuilder;
use raw_window_handle::HasWindowHandle;
use std::path::PathBuf;

impl<'a, W: HasWindowHandle> BlockingPickDirectoryDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let dialog = DialogBuilder::file()
            .set_title(self.title)
            .set_owner(&self.window)
            .open_single_dir();

        match dialog.show() {
            Ok(maybe_path) => Ok(maybe_path),
            Err(e) => Err(BlockingDialogError::NativeDialog(e)),
        }
    }
}
