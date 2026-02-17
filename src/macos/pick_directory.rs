// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickDirectoryDialog};
use block2::StackBlock;
use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSModalResponseOK, NSOpenPanel, NSView};
use objc2_foundation::NSString;
use raw_window_handle::RawWindowHandle;
use std::{path::PathBuf, sync::mpsc};

impl<'a> BlockingPickDirectoryDialog<'a> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let Some(mtm) = MainThreadMarker::new() else {
            return Err(BlockingDialogError::NotOnMainThread);
        };

        let panel = NSOpenPanel::openPanel(mtm);
        panel.setTitle(Some(&NSString::from_str(self.title)));
        panel.setCanChooseFiles(false);
        panel.setCanChooseDirectories(true);
        panel.setAllowsMultipleSelection(false);

        let RawWindowHandle::AppKit(handle) = self.window.as_raw() else {
            return Err(BlockingDialogError::UnsupportedWindowingSystem);
        };

        let (tx, rx) = mpsc::channel();
        let handler = StackBlock::new(move |resp| {
            let _ = tx.send(resp);
        });
        let handler = handler.copy();

        let ns_view = handle.ns_view.as_ptr();
        let ns_view = unsafe { Retained::from_raw(ns_view as *mut NSView) }.unwrap();
        let ns_window = ns_view.window().unwrap();

        panel.beginSheetModalForWindow_completionHandler(&ns_window, &handler);

        let resp = rx.recv().unwrap();

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
