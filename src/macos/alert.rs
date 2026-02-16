// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingAlertDialog, BlockingDialogError, BlockingDialogLevel};
use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSAlert, NSAlertStyle, NSImage, NSView};
use objc2_foundation::NSString;
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

impl<'a> BlockingAlertDialog<'a> {
    pub fn show(&self) -> Result<(), BlockingDialogError> {
        let Some(mtm) = MainThreadMarker::new() else {
            return Err(BlockingDialogError::NotOnMainThread);
        };

        let style = get_ns_alert_style(self.level);
        let icon = get_ns_alert_icon(self.level);

        let ns_alert = NSAlert::new(mtm);
        ns_alert.setMessageText(&NSString::from_str(self.title));
        ns_alert.setInformativeText(&NSString::from_str(self.message));
        ns_alert.setAlertStyle(style);

        if let Some(icon) = icon {
            unsafe { ns_alert.setIcon(Some(icon.as_ref())) }
        }

        if let Some(window) = &self.window
            && let RawWindowHandle::AppKit(handle) = window.as_raw()
        {
            let ns_view = handle.ns_view.as_ptr();
            let ns_view = unsafe { Retained::from_raw(ns_view as *mut NSView) }.unwrap();
            let ns_window = ns_view.window().unwrap();

            ns_alert.beginSheetModalForWindow_completionHandler(&ns_window, None);
        } else {
            let _ = ns_alert.runModal();
        }

        Ok(())
    }
}
