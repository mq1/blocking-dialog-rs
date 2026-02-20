// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingConfirmDialog, BlockingDialogError, BlockingDialogLevel};
use ashpd::desktop::{
    Icon,
    notification::{Button, Notification, NotificationProxy, Priority},
};
use futures::StreamExt;
use futures::executor::block_on;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use uuid::Uuid;

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingConfirmDialog<'a, W> {
    pub fn show(&self) -> Result<bool, BlockingDialogError> {
        let notification_id = Uuid::new_v4().to_string();

        let icon = match self.level {
            BlockingDialogLevel::Info => Icon::with_names(&["dialog-information"]),
            BlockingDialogLevel::Warning => Icon::with_names(&["dialog-warning"]),
            BlockingDialogLevel::Error => Icon::with_names(&["dialog-error"]),
        };

        block_on(async {
            let proxy = NotificationProxy::new().await?;

            proxy
                .add_notification(
                    &notification_id,
                    Notification::new(self.title)
                        .body(self.message)
                        .priority(Priority::High)
                        .icon(icon)
                        .button(Button::new("OK", "ok"))
                        .button(Button::new("Cancel", "cancel")),
                )
                .await?;

            let Some(action) = proxy.receive_action_invoked().await?.next().await else {
                return Err(BlockingDialogError::Ashpd(ashpd::Error::NoResponse));
            };

            let yes = action.name() == "ok";
            proxy.remove_notification(&notification_id).await?;

            Ok(yes)
        })
    }
}
