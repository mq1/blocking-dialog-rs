// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::widen;
use crate::{BlockingDialogError, BlockingPickFilesDialogFilter, BlockingSaveFileDialog};
use raw_window_handle::{HandleError, HasWindowHandle, RawWindowHandle};
use std::path::PathBuf;
use windows::Win32::Foundation::HWND;
use windows::{
    Win32::UI::Controls::Dialogs::{GetSaveFileNameW, OFN_EXPLORER, OFN_OVERWRITEPROMPT, OPENFILENAMEW},
    core::{PCWSTR, PWSTR},
};

fn get_filter_utf16(filter: &[BlockingPickFilesDialogFilter]) -> Vec<u16> {
    let mut s = String::new();

    for entry in filter {
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

impl<'a, W: HasWindowHandle> BlockingSaveFileDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let title_wide = widen(self.title);
        let filter_wide = get_filter_utf16(&self.filter);

        let w = match self.window.window_handle() {
            Ok(w) => w,
            Err(err) => return Err(BlockingDialogError::Handle(err)),
        };

        let RawWindowHandle::Win32(handle) = w.as_raw() else {
            return Err(BlockingDialogError::Handle(HandleError::NotSupported));
        };

        let hwnd = HWND(handle.hwnd.get() as *mut _);

        let mut file_buffer = vec![0u16; 4096];

        // Set default filename if provided
        if let Some(default_filename) = self.default_filename {
            let default_wide: Vec<u16> = default_filename
                .encode_utf16()
                .collect();
            file_buffer[..default_wide.len()].copy_from_slice(&default_wide);
        }

        let yes = unsafe {
            let mut ofn = OPENFILENAMEW {
                lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
                hwndOwner: hwnd,
                lpstrFilter: PCWSTR(filter_wide.as_ptr()),
                lpstrFile: PWSTR(file_buffer.as_mut_ptr()),
                nMaxFile: file_buffer.len() as u32,
                lpstrTitle: PCWSTR(title_wide.as_ptr()),
                Flags: OFN_EXPLORER | OFN_OVERWRITEPROMPT,
                ..Default::default()
            };

            GetSaveFileNameW(&mut ofn).as_bool()
        };

        if yes {
            let end = file_buffer.iter().position(|&c| c == 0).unwrap_or(file_buffer.len());
            let path = String::from_utf16_lossy(&file_buffer[..end]);
            Ok(Some(PathBuf::from(path)))
        } else {
            Ok(None)
        }
    }
}
