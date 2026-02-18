// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickDirectoryDialog};
use block2::RcBlock;
use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSApplication, NSModalResponseOK, NSOpenPanel, NSView};
use objc2_foundation::NSString;
use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle, RawWindowHandle};
use std::path::PathBuf;

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingPickDirectoryDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let Some(mtm) = MainThreadMarker::new() else {
            return Err(BlockingDialogError::NotOnMainThread);
        };

        let panel = NSOpenPanel::openPanel(mtm);
        panel.setTitle(Some(&NSString::from_str(self.title)));
        panel.setCanChooseFiles(false);
        panel.setCanChooseDirectories(true);
        panel.setAllowsMultipleSelection(false);

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

        let ns_view = w.ns_view.as_ptr();
        let ns_view = unsafe { Retained::retain_autoreleased(ns_view as *mut NSView) }.unwrap();
        let ns_window = ns_view.window().unwrap();

        panel.beginSheetModalForWindow_completionHandler(&ns_window, &handler);
        let resp = NSApplication::sharedApplication(mtm).runModalForWindow(&ns_window);

        let mut paths = Vec::new();

        if resp == NSModalResponseOK {
            for url in panel.URLs() {
                if let Some(path) = url.path() {
                    let path = path.as_ref() as &NSString;
                    paths.push(PathBuf::from(path.to_string()))
                }
            }
        }

        Ok(paths.pop())
    }
}
