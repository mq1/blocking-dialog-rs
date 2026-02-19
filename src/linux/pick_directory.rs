// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickDirectoryDialog};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::path::PathBuf;

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingPickDirectoryDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let dialog = zenity_rs::file_select()
            .title(self.title)
            .directory(true)
            .multiple(false);

        let res = dialog.show()?;

        if let zenity_rs::FileSelectResult::Selected(path) = res {
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }
}
