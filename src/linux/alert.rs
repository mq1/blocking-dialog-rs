// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingAlertDialog, BlockingDialogError, BlockingDialogLevel};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingAlertDialog<'a, W> {
    pub fn show(&self) -> Result<(), BlockingDialogError> {
        let dialog = match self.level {
            BlockingDialogLevel::Info => zenity_rs::info(self.message),
            BlockingDialogLevel::Warning => zenity_rs::warning(self.message),
            BlockingDialogLevel::Error => zenity_rs::error(self.message),
        };

        let dialog = dialog
            .title(self.title)
            .buttons(zenity_rs::ButtonPreset::Ok);

        let _ = dialog.show()?;

        Ok(())
    }
}
