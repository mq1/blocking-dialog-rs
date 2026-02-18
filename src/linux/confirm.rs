// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingConfirmDialog, BlockingDialogError, BlockingDialogLevel};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

fn get_rfd_dialog_level(level: BlockingDialogLevel) -> MessageLevel {
    match level {
        BlockingDialogLevel::Info => MessageLevel::Info,
        BlockingDialogLevel::Warning => MessageLevel::Warning,
        BlockingDialogLevel::Error => MessageLevel::Error,
    }
}

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingConfirmDialog<'a, W> {
    pub fn show(&self) -> Result<bool, BlockingDialogError> {
        let result = MessageDialog::new()
            .set_level(get_rfd_dialog_level(self.level))
            .set_title(self.title)
            .set_description(self.message)
            .set_buttons(MessageButtons::OkCancel)
            .set_parent(&self.window)
            .show();

        Ok(result == MessageDialogResult::Ok)
    }
}
