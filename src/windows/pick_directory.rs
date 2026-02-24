// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{unwiden, widen};
use crate::{BlockingDialogError, BlockingPickDirectoryDialog};
use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle, RawWindowHandle};
use std::path::PathBuf;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::UI::Shell::SHGetPathFromIDListW;
use windows::Win32::UI::Shell::{
    BIF_NEWDIALOGSTYLE, BIF_RETURNONLYFSDIRS, BROWSEINFOW, SHBrowseForFolderW,
};
use windows::core::PCWSTR;

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingPickDirectoryDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let w = self
            .window
            .window_handle()
            .map_err(BlockingDialogError::Handle)?;

        let RawWindowHandle::Win32(handle) = w.as_raw() else {
            return Err(BlockingDialogError::Handle(HandleError::NotSupported));
        };

        let hwnd = HWND(handle.hwnd.get() as *mut _);

        let title_wide = widen(self.title);

        let mut browse_info = BROWSEINFOW {
            hwndOwner: hwnd,
            lpszTitle: PCWSTR(title_wide.as_ptr()),
            ulFlags: BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE,
            ..Default::default()
        };

        let raw_pidl = unsafe { SHBrowseForFolderW(&mut browse_info) };

        if raw_pidl.is_null() {
            return Ok(None);
        }

        let pidl_guard = Pidl(raw_pidl as *mut _);

        let mut pszpath = [0u16; 260];
        let success = unsafe { SHGetPathFromIDListW(pidl_guard.0 as *const _, &mut pszpath) };

        if success.as_bool() {
            let path = unwiden(pszpath);
            Ok(Some(PathBuf::from(path)))
        } else {
            Ok(None)
        }
    }
}

struct Pidl(*mut std::ffi::c_void);
impl Drop for Pidl {
    fn drop(&mut self) {
        unsafe { CoTaskMemFree(Some(self.0)) };
    }
}
