// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickFilesDialogFilter, BlockingSaveFileDialog};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use rfd::FileDialog;
use std::path::PathBuf;

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingSaveFileDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let mut dialog = FileDialog::new()
            .set_title(self.title)
            .set_parent(&self.window);

        if let Some(default_filename) = self.default_filename {
            dialog = dialog.set_file_name(default_filename);
        }

        for filter in self.filter {
            dialog = dialog.add_filter(filter.name, filter.extensions);
        }

        let result = dialog.save_file();
        Ok(result)
    }
}
