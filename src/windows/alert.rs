// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::widen;
use crate::{BlockingAlertDialog, BlockingDialogError, BlockingDialogLevel};
use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle, RawWindowHandle};
use std::ffi::c_void;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK, MESSAGEBOX_STYLE, MessageBoxW,
};
use windows::core::PCWSTR;

fn get_utype(level: BlockingDialogLevel) -> MESSAGEBOX_STYLE {
    match level {
        BlockingDialogLevel::Info => MB_OK | MB_ICONINFORMATION,
        BlockingDialogLevel::Warning => MB_OK | MB_ICONWARNING,
        BlockingDialogLevel::Error => MB_OK | MB_ICONERROR,
    }
}

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingAlertDialog<'a, W> {
    pub fn show(&self) -> Result<(), BlockingDialogError> {
        let title_wide = widen(self.title);
        let message_wide = widen(self.message);

        let w = self
            .window
            .window_handle()
            .map_err(BlockingDialogError::Handle)?;

        let RawWindowHandle::Win32(handle) = w.as_raw() else {
            return Err(BlockingDialogError::Handle(HandleError::NotSupported));
        };

        let hwnd = handle.hwnd.get() as isize;
        let hwnd = HWND(hwnd as *mut c_void);
        let utype = get_utype(self.level);

        unsafe {
            let _ = MessageBoxW(
                Some(hwnd),
                PCWSTR(message_wide.as_ptr()),
                PCWSTR(title_wide.as_ptr()),
                utype,
            );
        }

        Ok(())
    }
}
