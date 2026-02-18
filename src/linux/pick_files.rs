// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickFilesDialog};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use rfd::FileDialog;
use std::path::PathBuf;

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingPickFilesDialog<'a, W> {
    pub fn show(&self) -> Result<Vec<PathBuf>, BlockingDialogError> {
        let mut dialog = FileDialog::new()
            .set_title(self.title)
            .set_parent(&self.window);

        for entry in self.filter {
            dialog = dialog.add_filter(entry.name, entry.extensions);
        }

        if self.multiple {
            match dialog.pick_files() {
                Some(files) => Ok(files),
                None => Ok(Vec::new()),
            }
        } else {
            match dialog.pick_file() {
                Some(file) => Ok(vec![file]),
                None => Ok(Vec::new()),
            }
        }
    }
}
