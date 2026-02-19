// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickFilesDialog};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::path::PathBuf;

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingPickFilesDialog<'a, W> {
    pub fn show(&self) -> Result<Vec<PathBuf>, BlockingDialogError> {
        let mut dialog = zenity_rs::file_select()
            .title(self.title)
            .directory(false)
            .multiple(self.multiple);

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

        match res {
            zenity_rs::FileSelectResult::Selected(path) => Ok(vec![path]),
            zenity_rs::FileSelectResult::SelectedMultiple(paths) => Ok(paths),
            _ => Ok(Vec::new()),
        }
    }
}
