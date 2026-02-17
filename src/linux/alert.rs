// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingAlertDialog, BlockingDialogError, BlockingDialogLevel};
use native_dialog::{DialogBuilder, MessageLevel};
use raw_window_handle::HasWindowHandle;

fn get_native_dialog_level(level: BlockingDialogLevel) -> MessageLevel {
    match level {
        BlockingDialogLevel::Info => MessageLevel::Info,
        BlockingDialogLevel::Warning => MessageLevel::Warning,
        BlockingDialogLevel::Error => MessageLevel::Error,
    }
}

impl<'a, W: HasWindowHandle> BlockingAlertDialog<'a, W> {
    pub fn show(&self) -> Result<(), BlockingDialogError> {
        let dialog = DialogBuilder::message()
            .set_title(self.title)
            .set_text(self.message)
            .set_level(get_native_dialog_level(self.level))
            .set_owner(&self.window)
            .alert();

        match dialog.show() {
            Ok(()) => Ok(()),
            Err(e) => Err(BlockingDialogError::NativeDialog(e)),
        }
    }
}
