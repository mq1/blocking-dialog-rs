// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingConfirmDialog, BlockingDialogError, BlockingDialogLevel};
use block2::RcBlock;
use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{
    NSAlert, NSAlertSecondButtonReturn, NSAlertStyle, NSApplication, NSImage, NSTintProminence,
    NSView,
};
use objc2_foundation::{NSString, ns_string};
use raw_window_handle::{HandleError, HasWindowHandle, RawWindowHandle};

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

impl<'a, W: HasWindowHandle> BlockingConfirmDialog<'a, W> {
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
            unsafe { ns_alert.setIcon(Some(icon.as_ref())) }
        }

        let w = match self.window.window_handle() {
            Ok(w) => w,
            Err(err) => return Err(BlockingDialogError::Handle(err)),
        };

        let RawWindowHandle::AppKit(w) = w.as_raw() else {
            return Err(BlockingDialogError::Handle(HandleError::NotSupported));
        };

        let handler = RcBlock::new(move |resp| {
            NSApplication::sharedApplication(mtm).stopModalWithCode(resp);
        });
        let handler = handler.copy();

        let ns_view = w.ns_view.as_ptr();
        let ns_view = unsafe { Retained::retain_autoreleased(ns_view as *mut NSView) }.unwrap();
        let ns_window = ns_view.window().unwrap();

        ns_alert.beginSheetModalForWindow_completionHandler(&ns_window, Some(&handler));
        let resp = NSApplication::sharedApplication(mtm).runModalForWindow(&ns_window);

        Ok(resp == NSAlertSecondButtonReturn)
    }
}
