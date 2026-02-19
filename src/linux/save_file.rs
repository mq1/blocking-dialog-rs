// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingSaveFileDialog};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::path::PathBuf;

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingSaveFileDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let mut dialog = zenity_rs::file_select()
            .title(self.title)
            .directory(false)
            .multiple(false)
            .save(true);

        if let Some(default_filename) = self.default_filename {
            dialog = dialog.filename(default_filename);
        }

        for entry in self.filter {
            let patterns = entry
                .extensions
                .iter()
                .map(|ext| format!("*.{}", ext))
                .collect::<Vec<_>>();

            let filter = zenity_rs::FileFilter {
                name: entry.name.to_string(),
                patterns,
            };

            dialog = dialog.add_filter(filter);
        }

        let res = dialog.show()?;

        if let zenity_rs::FileSelectResult::Selected(path) = res {
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }
}
