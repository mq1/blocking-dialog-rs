// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod alert;
pub mod confirm;

pub fn widen<S: AsRef<str>>(s: S) -> Vec<u16> {
    s.as_ref()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect()
}
