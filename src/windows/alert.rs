// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::widen;
use crate::{BlockingAlertDialog, BlockingDialogError, BlockingDialogLevel};
use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle, RawWindowHandle};
use std::ffi::c_void;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK, MB_SETFOREGROUND, MB_TOPMOST,
    MESSAGEBOX_STYLE, MessageBoxW,
};
use windows::core::PCWSTR;

fn get_utype(level: BlockingDialogLevel) -> MESSAGEBOX_STYLE {
    let level = match level {
        BlockingDialogLevel::Info => MB_ICONINFORMATION,
        BlockingDialogLevel::Warning => MB_ICONWARNING,
        BlockingDialogLevel::Error => MB_ICONERROR,
    };

    level | MB_OK | MB_TOPMOST | MB_SETFOREGROUND
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

        unsafe {
            let _ = MessageBoxW(
                None,
                PCWSTR(message_wide.as_ptr()),
                PCWSTR(title_wide.as_ptr()),
                get_utype(self.level),
            );
        }

        Ok(())
    }
}
