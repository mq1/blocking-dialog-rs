// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::widen;
use crate::{BlockingDialogError, BlockingPickFilesDialog, BlockingPickFilesDialogFilter};
use raw_window_handle::RawWindowHandle;
use std::path::PathBuf;
use windows::Win32::Foundation::HWND;
use windows::{
    Win32::UI::Controls::Dialogs::{
        GetOpenFileNameW, OFN_ALLOWMULTISELECT, OFN_EXPLORER, OFN_FILEMUSTEXIST, OFN_PATHMUSTEXIST,
        OPENFILENAMEW,
    },
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

fn parse_multi_select(buffer: &[u16]) -> Vec<PathBuf> {
    let parts: Vec<String> = buffer
        .split(|c| *c == 0)
        .filter(|s| !s.is_empty())
        .map(|s| String::from_utf16_lossy(s))
        .collect();

    if parts.is_empty() {
        return vec![];
    }

    // Single file selected
    if parts.len() == 1 {
        return vec![PathBuf::from(&parts[0])];
    }

    // Multi-select: first part is directory, rest are filenames
    let dir = PathBuf::from(&parts[0]);
    parts[1..].iter().map(|f| dir.join(f)).collect()
}

impl<'a> BlockingPickFilesDialog<'a> {
    pub fn show(&self) -> Result<Vec<PathBuf>, BlockingDialogError> {
        let title_wide = widen(self.title);
        let filter_wide = get_filter_utf16(&self.filter);

        let RawWindowHandle::Win32(handle) = self.window.as_raw() else {
            return Err(BlockingDialogError::UnsupportedWindowingSystem);
        };

        let hwnd = HWND(handle.hwnd.get() as *mut _);

        let mut file_buffer = vec![0u16, 32_768];

        let mut flags = OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_EXPLORER;
        if self.multiple {
            flags |= OFN_ALLOWMULTISELECT;
        }

        let yes = unsafe {
            let mut ofn = OPENFILENAMEW {
                lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
                hwndOwner: hwnd,
                lpstrFilter: PCWSTR(filter_wide.as_ptr()),
                lpstrFile: PWSTR(file_buffer.as_mut_ptr()),
                nMaxFile: file_buffer.len() as u32,
                lpstrTitle: PCWSTR(title_wide.as_ptr()),
                Flags: flags,
                ..Default::default()
            };

            GetOpenFileNameW(&mut ofn).as_bool()
        };

        if yes {
            Ok(parse_multi_select(&file_buffer))
        } else {
            Ok(Vec::new())
        }
    }
}
