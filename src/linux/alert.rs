// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingAlertDialog, BlockingDialogError, BlockingDialogLevel};
use raw_window_handle::HasWindowHandle;
use rfd::{MessageDialog, MessageLevel};

fn get_rfd_dialog_level(level: BlockingDialogLevel) -> MessageLevel {
    match level {
        BlockingDialogLevel::Info => MessageLevel::Info,
        BlockingDialogLevel::Warning => MessageLevel::Warning,
        BlockingDialogLevel::Error => MessageLevel::Error,
    }
}

impl<'a, W: HasWindowHandle> BlockingAlertDialog<'a, W> {
    pub fn show(&self) -> Result<(), BlockingDialogError> {
        let _ = MessageDialog::new()
            .set_level(get_rfd_dialog_level(self.level))
            .set_title(self.title)
            .set_description(self.message)
            .set_parent(&self.window)
            .show();

        Ok(())
    }
}
