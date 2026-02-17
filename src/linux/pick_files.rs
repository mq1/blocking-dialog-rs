// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickFilesDialog};
use native_dialog::DialogBuilder;
use raw_window_handle::HasWindowHandle;
use std::path::PathBuf;

impl<'a, W: HasWindowHandle> BlockingPickFilesDialog<'a, W> {
    pub fn show(&self) -> Result<Vec<PathBuf>, BlockingDialogError> {
        let mut dialog = DialogBuilder::file()
            .set_title(self.title)
            .set_owner(&self.window);

        for entry in self.filter {
            dialog = dialog.add_filter(entry.name, entry.extensions);
        }

        if self.multiple {
            let dialog = dialog.open_multiple_file();

            match dialog.show() {
                Ok(paths) => Ok(paths),
                Err(err) => Err(BlockingDialogError::NativeDialog(err)),
            }
        } else {
            let dialog = dialog.open_single_file();

            match dialog.show() {
                Ok(path) => Ok(path.into_iter().collect()),
                Err(err) => Err(BlockingDialogError::NativeDialog(err)),
            }
        }
    }
}
