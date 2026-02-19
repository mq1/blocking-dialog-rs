// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingConfirmDialog, BlockingDialogError, BlockingDialogLevel};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingConfirmDialog<'a, W> {
    pub fn show(&self) -> Result<bool, BlockingDialogError> {
        let dialog = match self.level {
            BlockingDialogLevel::Info => zenity_rs::info(self.message),
            BlockingDialogLevel::Warning => zenity_rs::warning(self.message),
            BlockingDialogLevel::Error => zenity_rs::error(self.message),
        };

        let dialog = dialog
            .title(self.title)
            .buttons(zenity_rs::ButtonPreset::OkCancel);

        let yes = dialog.show()? == zenity_rs::DialogResult::Button(1);

        Ok(yes)
    }
}
