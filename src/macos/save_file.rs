// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickFilesDialogFilter, BlockingSaveFileDialog};
use block2::RcBlock;
use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSApplication, NSModalResponseOK, NSSavePanel, NSView};
use objc2_foundation::{NSArray, NSString};
use objc2_uniform_type_identifiers::UTType;
use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle, RawWindowHandle};
use std::path::PathBuf;

fn get_filter(filter: &[BlockingPickFilesDialogFilter]) -> Retained<NSArray<UTType>> {
    let mut vec = Vec::new();

    for entry in filter {
        for ext in entry.extensions {
            let ext = NSString::from_str(ext);
            let uttype = UTType::typeWithFilenameExtension(&ext);
            if let Some(uttype) = uttype {
                vec.push(uttype)
            }
        }
    }

    NSArray::from_retained_slice(vec.as_slice())
}

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingSaveFileDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let Some(mtm) = MainThreadMarker::new() else {
            return Err(BlockingDialogError::NotOnMainThread);
        };

        let panel = NSSavePanel::savePanel(mtm);
        panel.setTitle(Some(&NSString::from_str(self.title)));
        panel.setAllowedContentTypes(&get_filter(self.filter));

        if let Some(default_filename) = &self.default_filename {
            let default_filename = NSString::from_str(default_filename);
            panel.setNameFieldStringValue(&default_filename);
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

        let ns_view = w.ns_view.as_ptr();
        let ns_view = unsafe { Retained::retain_autoreleased(ns_view as *mut NSView) }.unwrap();
        let ns_window = ns_view.window().unwrap();

        panel.beginSheetModalForWindow_completionHandler(&ns_window, &handler);
        let resp = NSApplication::sharedApplication(mtm).runModalForWindow(&ns_window);

        if resp == NSModalResponseOK
            && let Some(url) = panel.URL()
            && let Some(path) = url.path()
        {
            let path = path.as_ref() as &NSString;
            Ok(Some(PathBuf::from(path.to_string())))
        } else {
            Ok(None)
        }
    }
}
