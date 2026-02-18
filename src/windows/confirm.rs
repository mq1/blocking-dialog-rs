// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::widen;
use crate::{BlockingConfirmDialog, BlockingDialogError, BlockingDialogLevel};
use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
use windows::Win32::UI::WindowsAndMessaging::{
    MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MB_OKCANCEL, MESSAGEBOX_RESULT,
    MESSAGEBOX_STYLE, MessageBoxW,
};
use windows::core::PCWSTR;

fn get_utype(level: BlockingDialogLevel) -> MESSAGEBOX_STYLE {
    match level {
        BlockingDialogLevel::Info => MB_OKCANCEL | MB_ICONINFORMATION,
        BlockingDialogLevel::Warning => MB_OKCANCEL | MB_ICONWARNING,
        BlockingDialogLevel::Error => MB_OKCANCEL | MB_ICONERROR,
    }
}

impl<'a, W: HasWindowHandle + HasDisplayHandle> BlockingConfirmDialog<'a, W> {
    pub fn show(&self) -> Result<bool, BlockingDialogError> {
        let com_initialized = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).is_ok() };

        let title_wide = widen(self.title);
        let message_wide = widen(self.message);

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
        let utype = get_utype(self.level);

        let yes = unsafe {
            let res = MessageBoxW(
                Some(hwnd),
                PCWSTR(message_wide.as_ptr()),
                PCWSTR(title_wide.as_ptr()),
                utype,
            );

            if com_initialized {
                CoUninitialize();
            }

            res == MESSAGEBOX_RESULT(1)
        };

        Ok(yes)
    }
}
