// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::widen;
use crate::{BlockingAlertDialog, BlockingDialogError, BlockingDialogLevel};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK, MESSAGEBOX_STYLE, MessageBoxW,
};
use windows::core::PCWSTR;

fn get_utype(level: BlockingDialogLevel) -> MESSAGEBOX_STYLE {
    let flags = match level {
        BlockingDialogLevel::Info => MB_OK | MB_ICONINFORMATION,
        BlockingDialogLevel::Warning => MB_OK | MB_ICONWARNING,
        BlockingDialogLevel::Error => MB_OK | MB_ICONERROR,
    };

    MESSAGEBOX_STYLE(flags)
}

impl<'a> BlockingAlertDialog<'a> {
    pub fn show(&self) -> Result<(), BlockingDialogError> {
        let title_wide = widen(self.title);
        let message_wide = widen(self.message);

        let hwnd = if let Some(handle) = self.window
            && let RawWindowHandle::Win32(handle) = handle.as_raw()
        {
            Some(HWND(handle.hwnd.get() as *mut _));
        } else {
            None
        };

        let utype = get_utype(self.level);

        unsafe {
            let _ = MessageBoxW(
                hwnd,
                PCWSTR(message_wide.as_ptr()),
                PCWSTR(title_wide.as_ptr()),
                utype,
            );
        }

        Ok(())
    }
}
