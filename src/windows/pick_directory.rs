// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{unwiden, widen};
use crate::{BlockingDialogError, BlockingPickDirectoryDialog};
use raw_window_handle::{HandleError, HasWindowHandle, RawWindowHandle};
use std::path::PathBuf;
use std::ptr::null_mut;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
use windows::Win32::UI::Shell::SHGetPathFromIDListW;
use windows::Win32::UI::Shell::{
    BIF_NEWDIALOGSTYLE, BIF_RETURNONLYFSDIRS, BROWSEINFOA, BROWSEINFOW, SHBrowseForFolderW,
};
use windows::core::PCWSTR;

impl<'a, W: HasWindowHandle> BlockingPickDirectoryDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        unsafe {
            if CoInitializeEx(None, COINIT_APARTMENTTHREADED).is_err() {
                return Err(BlockingDialogError::CouldNotInitializeCOM);
            }
        }

        let w = match self.window.window_handle() {
            Ok(w) => w,
            Err(err) => return Err(BlockingDialogError::Handle(err)),
        };

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

        let pidl = unsafe { SHBrowseForFolderW(&mut browse_info) };

        if pidl.is_null() {
            CoUninitialize();
            return Ok(None);
        }

        let mut path_buffer = [0u16; 260];
        let success = unsafe { SHGetPathFromIDListW(pidl, PWSTR(path_buffer.as_mut_ptr())) };

        unsafe {
            CoTaskMemFree(Some(pidl as *const _));
            CoUninitialize();
        }

        if success.as_bool() {
            let path = unwiden(path_buffer);
            Ok(Some(PathBuf::from(path)))
        } else {
            Ok(None)
        }
    }
}
