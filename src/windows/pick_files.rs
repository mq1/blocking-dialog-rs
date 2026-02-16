// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BlockingDialogError, BlockingPickFileDialog, BlockingPickFileDialogFilter};
use std::path::PathBuf;
use windows::Win32::Foundation::HWND;
use windows::{
    Win32::UI::Controls::Dialogs::{
        GetOpenFileNameW, OFN_FILEMUSTEXIST, OFN_PATHMUSTEXIST, OPENFILENAMEW,
    },
    core::{PCWSTR, PWSTR},
};

fn get_filter_utf16(filter: &[BlockingPickFileDialogFilter]) -> Vec<u16> {
    let mut s = String::new();

    for entry in filters {
        s.push_str(&entry.name);
        s.push(char::from(0));

        let extensions = entry
            .extensions
            .iter()
            .map(|ext| format!("*.{ext}"))
            .collect::<Vec<_>>()
            .join(";");

        s.push_str(&extensions);
        s.push(char::from(0));
    }

    widen(s)
}

impl<'a> BlockingPickFileDialog<'a> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let title_wide = widen(self.title);
        let message_wide = widen(self.message);

        let hwnd = if let Some(handle) = self.window
            && let RawWindowHandle::Win32(handle) = handle.as_raw()
        {
            HWND(handle.hwnd.get() as *mut _)
        } else {
            HWND(0)
        };

        let mut file_buffer = [0u16; 260];

        let yes = unsafe {
            let mut ofn = OPENFILENAMEW {
                lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
                hwndOwner: hwnd,
                lpstrFilter: PCWSTR(filters_wide.as_ptr()),
                lpstrFile: PWSTR(file_buffer.as_mut_ptr()),
                nMaxFile: file_buffer.len() as u32,
                lpstrTitle: PCWSTR(title_wide.as_ptr()),
                Flags: OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST,
                ..Default::default()
            };

            GetOpenFileNameW(&mut ofn).as_bool()
        };

        if yes {
            let path = unwiden(&file_buffer);
            Ok(Some(PathBuf::from(path)))
        } else {
            Ok(None)
        }
    }
}
