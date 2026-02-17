// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingSaveFileDialog};
use native_dialog::DialogBuilder;
use raw_window_handle::HasWindowHandle;
use std::path::PathBuf;

impl<'a, W: HasWindowHandle> BlockingSaveFileDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let mut dialog = DialogBuilder::file()
            .set_title(self.title)
            .set_owner(&self.window);

        if let Some(default_filename) = self.default_filename {
            dialog = dialog.set_filename(default_filename);
        }

        for filter in self.filter {
            dialog = dialog.add_filter(filter.name, filter.extensions);
        }

        let dialog = dialog.save_single_file();

        match dialog.show() {
            Ok(maybe_path) => Ok(maybe_path),
            Err(err) => Err(BlockingDialogError::NativeDialog(err)),
        }
    }
}
