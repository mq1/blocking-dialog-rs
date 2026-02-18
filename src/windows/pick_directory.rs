// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{unwiden, widen};
use crate::{BlockingDialogError, BlockingPickDirectoryDialog};
use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle, RawWindowHandle};
use std::path::PathBuf;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
use windows::Win32::UI::Shell::SHGetPathFromIDListW;
use windows::Win32::UI::Shell::{
    BIF_NEWDIALOGSTYLE, BIF_RETURNONLYFSDIRS, BROWSEINFOW, SHBrowseForFolderW,
};
use windows::core::PCWSTR;

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingPickDirectoryDialog<'a, W> {
    pub fn show(&self) -> Result<Option<PathBuf>, BlockingDialogError> {
        let com_initialized = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).is_ok() };

        let w = match self.window.window_handle() {
            Ok(w) => w,
            Err(err) => {
                if com_initialized {
                    unsafe { CoUninitialize() };
                }
                return Err(BlockingDialogError::Handle(err));
            }
        };

        let RawWindowHandle::Win32(handle) = w.as_raw() else {
            if com_initialized {
                unsafe { CoUninitialize() };
            }
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
            if com_initialized {
                unsafe { CoUninitialize() };
            }
            return Ok(None);
        }

        let mut pszpath = [0u16; 260];
        let success = unsafe { SHGetPathFromIDListW(pidl, &mut pszpath) };

        unsafe {
            CoTaskMemFree(Some(pidl as *const _));
            if com_initialized {
                CoUninitialize();
            }
        }

        if success.as_bool() {
            let path = unwiden(pszpath);
            Ok(Some(PathBuf::from(path)))
        } else {
            Ok(None)
        }
    }
}
