// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::sync::mpsc;

use crate::{BlockingConfirmDialog, BlockingDialogError, BlockingDialogLevel};
use block2::StackBlock;
use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{
    NSAlert, NSAlertSecondButtonReturn, NSAlertStyle, NSImage, NSTintProminence, NSView,
};
use objc2_foundation::{NSString, ns_string};
use raw_window_handle::RawWindowHandle;

fn get_ns_alert_style(level: BlockingDialogLevel) -> NSAlertStyle {
    match level {
        BlockingDialogLevel::Info => NSAlertStyle::Informational,
        BlockingDialogLevel::Warning => NSAlertStyle::Warning,
        BlockingDialogLevel::Error => NSAlertStyle::Warning,
    }
}

fn get_ns_alert_icon(level: BlockingDialogLevel) -> Option<Retained<NSImage>> {
    match level {
        BlockingDialogLevel::Info => NSImage::imageWithSystemSymbolName_accessibilityDescription(
            &NSString::from_str("info.circle"),
            Some(&NSString::from_str("Info")),
        ),
        BlockingDialogLevel::Warning => {
            NSImage::imageWithSystemSymbolName_accessibilityDescription(
                &NSString::from_str("exclamationmark.triangle"),
                Some(&NSString::from_str("Warning")),
            )
        }
        BlockingDialogLevel::Error => NSImage::imageWithSystemSymbolName_accessibilityDescription(
            &NSString::from_str("multiply.circle"),
            Some(&NSString::from_str("Error")),
        ),
    }
}

impl<'a> BlockingConfirmDialog<'a> {
    pub fn show(&self) -> Result<bool, BlockingDialogError> {
        let Some(mtm) = MainThreadMarker::new() else {
            return Err(BlockingDialogError::NotOnMainThread);
        };

        let style = get_ns_alert_style(self.level);
        let icon = get_ns_alert_icon(self.level);

        let ns_alert = NSAlert::new(mtm);
        ns_alert.setMessageText(&NSString::from_str(self.title));
        ns_alert.setInformativeText(&NSString::from_str(self.message));
        ns_alert.setAlertStyle(style);
        let _cancel_btn = ns_alert.addButtonWithTitle(ns_string!("Cancel"));
        let ok_btn = ns_alert.addButtonWithTitle(ns_string!("OK"));
        ok_btn.setTintProminence(NSTintProminence::Primary);

        if let Some(icon) = icon {
            let icon = icon.downcast_ref();
            unsafe { ns_alert.setIcon(icon) }
        }

        let resp = if let Some(window) = &self.window
            && let RawWindowHandle::AppKit(handle) = window.as_raw()
        {
            let (tx, rx) = mpsc::channel();
            let handler = StackBlock::new(move |resp| {
                let _ = tx.send(resp);
            });
            let handler = handler.copy();

            let ns_view = handle.ns_view.as_ptr();
            let ns_view = unsafe { Retained::from_raw(ns_view as *mut NSView) }.unwrap();
            let ns_window = ns_view.window().unwrap();

            ns_alert.beginSheetModalForWindow_completionHandler(&ns_window, Some(&handler));

            rx.recv().unwrap()
        } else {
            ns_alert.runModal()
        };

        Ok(resp == NSAlertSecondButtonReturn)
    }
}
